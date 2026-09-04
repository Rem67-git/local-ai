# DECISIONS — Technology and Architectural Choices

This document records key technology decisions made during the project, following the Architecture Decision Record (ADR) pattern. Each decision includes context, alternatives considered, the choice made, and rationale.

---

## Decision 1: Desktop Application Framework

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Need a cross-platform desktop UI that is lightweight, can embed a Rust runtime for security-critical operations, and provides good integration with local filesystem and system APIs. Windows 11 Pro is the primary target, but cross-platform support is valuable.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Tauri + React/TypeScript** | Lightweight; Rust core for security & perf; reusable web UI patterns; native Windows/Mac/Linux support; zero-overhead desktop integration | Newer ecosystem; smaller community than Electron |
| Electron | Mature; large ecosystem; many examples | Heavy (70+ MB baseline); all Chromium overhead; no Rust core; less suitable for security-critical paths |
| Qt + Rust | Native performance; Qt maturity | Complex build; licensing considerations; steeper learning curve |
| CLI only | Simpler; no UI overhead | Harder for non-technical users; less real-time feedback |

### Decision
**Tauri + React/TypeScript**

### Rationale
- Tauri's lightweight architecture aligns with privacy-first principles (no shipping a full browser)
- Rust backend enables implementing the security-critical policy engine and agent runtime in a memory-safe language
- React/TypeScript provides familiar UI patterns; existing frontend best practices apply
- Good Windows 11 support (AppContainer for sandboxing, native window management)
- Smaller footprint allows bundling with local LLM models (important for offline-first)

---

## Decision 2: Backend Runtime Language

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
The core runtime (agent loop, policy engine, tool router, sandbox) handles untrusted input and makes security-critical decisions. This must be robust, memory-safe, and performant.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Rust** | Memory safety; performance; strong type system; suitable for security-critical code | Steeper learning curve; slower compile times |
| Python | Fast to prototype; ML/AI libraries abundant; easy to read | Memory overhead; GIL limitations; not suitable for security gates |
| Go | Lightweight; good concurrency; faster than Python | Less memory safety than Rust; GC pauses; weaker type system |
| C++ | Performance; control; large ecosystem | Manual memory management; harder to audit for security |

### Decision
**Rust for core runtime**; Python only where unavoidable (ML embeddings, specialized transformations).

### Rationale
- Policy engine, agent executor, and sandbox provider must be memory-safe (no buffer overflows, use-after-free)
- Rust's type system prevents whole classes of bugs (null pointer dereference, race conditions)
- Performance critical for agent loop (lower latency = better UX)
- Tauri already uses Rust backend; consolidating on one language reduces cross-language complexity

**Exception**: Python libraries may be wrapped for embeddings (e.g., sentence-transformers) where Rust equivalents are immature or unavailable.

---

## Decision 3: Local LLM Runtime

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
The system must run LLM inference entirely locally without cloud API calls. Choice of runtime affects model compatibility, performance, and bundling strategy.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Abstraction over llama.cpp + Ollama** | Supports GGUF/GPTQ quantized models; good Windows support; both free/open; allows switching | Requires integration with both; user must choose one |
| llama.cpp only | Minimal, focused; good performance; single C++ codebase | Less flexible; no HTTP API (harder for IPC from Tauri) |
| Ollama only | REST API (easy IPC); good model management UI; simple | Slightly heavier; fewer quantization options |
| OpenAI API (cloud) | Mature; reliable | **Violates offline-first principle** — cloud dependency |
| HuggingFace Transformers (Python) | Vast model zoo; Transformers ecosystem | Heavy; requires Python runtime; slower on CPU |

### Decision
**Abstraction layer supporting both llama.cpp and Ollama**, defaulting to Ollama for better UX (REST API, model download UI).

### Rationale
- Ollama provides a user-friendly REST API; Tauri can query it via HTTP without FFI complexity
- llama.cpp option available for advanced users who need raw performance or specific configurations
- No model is hardcoded; users can swap models (Mistral, Llama, Qwen, etc.)
- Both projects are actively maintained and compatible with quantized models (GGUF)
- Abstraction allows future substitution without rewriting agent code

**LLM not chosen**: Specific model selection deferred to Phase 1 (depends on hardware, user preference, license).

---

## Decision 4: Database and State Persistence

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Agent runtime must persist mission state, memory, tool observations, and checkpoints for resumability and offline operation. Require ACID guarantees and local-only storage.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **SQLite** | Zero setup; ACID; file-based; works offline; mature; standard for desktop | Single-writer; no built-in replication |
| PostgreSQL | Powerful; great for scaling; transactions | Requires separate server; overkill for local agent |
| MongoDB | Document-oriented; flexible schema | Not ideal for offline; requires server |
| RocksDB | Embedded; high performance; log-structured | Fewer transaction guarantees; steeper learning curve |

### Decision
**SQLite** for operational state and memory; supplemented by **FAISS** for semantic embeddings (read-only after indexing).

### Rationale
- SQLite is zero-configuration; no separate database server to manage
- File-based persistence maps naturally to offline-first (entire DB in one directory)
- ACID guarantees prevent state corruption if agent crashes mid-mission
- Existing migrations tools (sqlx, refinery) allow schema versioning
- FAISS (vector library) handles similarity search for memory retrieval without requiring a separate database

---

## Decision 5: Sandboxing and Isolation

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Agent must execute untrusted code (user scripts, tool operations, shell commands) without compromising the host system. Isolation strategy depends on OS.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Native per-OS isolation (first target)** | Leverages OS capabilities; performant; no heavy containers | Must implement Windows AppContainer + Unix chroot separately |
| Docker/Podman | Single approach cross-platform; strong isolation | Heavy; adds dependency; not suitable for every tool call |
| No sandboxing | Simplicity; performance | **Security risk** — agent can damage host |
| WASM | Sandboxed; portable | Overkill for most tools; limited filesystem access patterns |

### Decision
**Phase 1 target**: Native per-OS isolation (Windows AppContainer, Unix chroot).  
**Phase 1 fallback**: Restricted subprocess with filesystem whitelist + permission checks.  
**Future**: Docker integration optional (Phase 5+).

### Rationale
- Native isolation uses OS capabilities without extra dependencies
- Windows AppContainer can restrict filesystem, registry, network; executes as same user (no privilege escalation)
- Unix chroot is lightweight and prevents path traversal
- If performance or complexity issues arise, Docker becomes available in later phases
- Fallback (restricted subprocess) is sufficient for MVP and can be upgraded incrementally

---

## Decision 6: Package Management and Build

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Project spans Rust, TypeScript/React, and potentially Python. Need consistent, reproducible builds and dependency management.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Rust: Cargo; TS: npm/pnpm; Python: pip + requirements.txt** | Native tooling per language; wide adoption; good caching | Multiple lock files; coordination needed |
| Monorepo (Turborepo) | Single dependency graph; easier refactoring | Adds abstraction; slower for multi-language projects |
| Bazel | Reproducible; powerful; cross-language | Steep learning curve; overkill for MVP |

### Decision
**Cargo for Rust; npm or pnpm for TypeScript; pip + requirements.txt for Python** (workspace structure per language).

### Rationale
- Each language has mature, standard tooling; no benefit to indirection
- Separate lock files (Cargo.lock, package-lock.json, requirements.txt) are version-controlled and committed
- Easier to troubleshoot (problems isolated to language ecosystem)
- Workspace boundaries are clear (fewer hidden dependencies)

---

## Decision 7: Testing Strategy

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Project security depends on correct policy enforcement, sandbox integrity, and agent reliability. Tests must cover units, integration, security, and offline scenarios.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Multi-layer (unit + integration + security + offline)** | Comprehensive coverage; catches regressions at each layer | More maintenance; longer CI time |
| Unit tests only | Fast; isolated | Misses integration bugs; won't catch sandbox failures |
| Manual testing | Flexible | Not reproducible; scales poorly |

### Decision
**Four test tiers**:
1. **Unit tests** (Rust: Criterion, TS: Jest, Python: pytest) — individual components
2. **Integration tests** (agent + tools + DB) — end-to-end mission flows
3. **Security tests** (path traversal, unauthorized access, network escape) — threat model validation
4. **Offline tests** (`local-ai offline-test` command) — no network, full mission completion

### Rationale
- Unit tests catch bugs early; fast feedback loop
- Integration tests validate data flow across components
- Security tests are non-negotiable for policy engine and sandbox
- Offline tests are the proof that privacy-first promise is kept

---

## Decision 8: Logging and Observability

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Agent runtime must be debuggable and observable. Logs must support offline replay, security audits, and user debugging without collecting data.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Structured JSON logs to local files** | Machine-readable; sortable; no telemetry; audit trail | Requires JSON parsing for reading; disk space |
| Plain text logs | Human-readable | Less queryable; harder to correlate |
| Cloud logging (DataDog, CloudWatch) | Centralized; powerful analytics | **Cloud dependency** — violates privacy-first |
| In-memory only | Fast; no disk I/O | Lost on crash; no audit trail |

### Decision
**Structured JSON logs to local files** (one file per mission, organized by date).

### Rationale
- Logs stay on user's machine (no privacy compromise)
- JSON format allows parsing and analysis tools
- One file per mission prevents log files from becoming unwieldy
- Logs can be reviewed by user for security audits, debugging
- No external dependency on logging infrastructure

---

## Decision 9: Vector Embeddings and RAG

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Memory manager must support semantic search (retrieval-augmented generation). Embeddings need to be computed and stored locally.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **FAISS (local library)** | Embeds in local index; fast search; open-source; Python bindings | Requires embedding model (e.g., sentence-transformers) |
| Ollama embeddings API | Piggybacked on existing LLM runtime; HTTP API | Limited embedding model choice |
| Cloud embedding service (OpenAI) | Accurate; maintained by provider | **Cloud dependency** — violates offline-first |
| No semantic search (keyword only) | Simpler; no model overhead | Less powerful memory recall; misses semantic similarity |

### Decision
**FAISS** for vector index; embedding model selected from open-source options (e.g., sentence-transformers / all-MiniLM) in Phase 8.

### Rationale
- FAISS is battle-tested and performant
- Embedding model runs locally (can reuse LLM runtime or standalone)
- No cloud dependency
- Integrates naturally with SQLite for metadata

---

## Decision 10: Configuration Management

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
System has multiple configuration layers: user preferences (UI), policy rules, sandbox settings, model selection, resource budgets. Must be editable, versionable, and not require restart.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **YAML/TOML files in config directory** | Human-readable; versionable; simple format | Must be parsed and validated; reload logic needed |
| JSON | Strict; validatable with schema | Less human-friendly |
| SQLite config table | Queryable; easy updates; transactional | Overkill for static config; mixing data/config types |
| Hard-coded defaults + env vars | Simple | Not user-editable without recompile |

### Decision
**TOML config files** (`~/.local-ai/config.toml` or similar), loaded at startup with validation. Hot-reload of policy rules on file change (agent observes, re-evaluates).

### Rationale
- TOML is human-editable, versioned in user's home directory
- Schema validation prevents invalid configurations
- Separating policy config from runtime state keeps concerns clear
- Changes to policy engine rules do not require app restart (hot-reload)

---

## Decision 11: Git and Version Control

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Project is under active development, requires version history, collaboration, and reproducible builds.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Git (GitHub/GitLab)** | Industry standard; distributed; excellent tooling | None for this context |
| Mercurial | Similar to Git; slightly cleaner history | Less adoption; fewer integrations |
| Fossil | All-in-one VCS + wiki + tickets | Less common; not required yet |

### Decision
**Git** with `.gitignore` excluding:
- Model files (`inference/models/**`)
- Lock files (local copies, Cargo.lock / package-lock.json committed)
- User config (`~/.local-ai/config.toml`)
- Database files (user's SQLite)
- Build artifacts (`target/`, `node_modules/`, `.build/`)
- Secrets (`.env`, `credentials.json`)

### Rationale
- Standard de facto; all developers familiar
- Easy to set up CI/CD
- Can isolate user data from source code via `.gitignore`

---

## Decision 12: Development Workflow and CI/CD

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Need reproducible builds, automated testing, and verification that code passes security gates before merging.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **GitHub Actions (or equivalent)** | Free; integrates with Git; easy to configure | Proprietary (GitHub-specific) |
| GitLab CI | Similar to GitHub Actions; good documentation | Requires GitLab hosting |
| Self-hosted CI (Jenkins) | Full control; works on any Git host | Requires infrastructure; more maintenance |

### Decision
**GitHub Actions** (or equivalent per platform), running:
1. Lint (clippy for Rust, eslint for TS, pylint for Python)
2. Unit tests (all languages)
3. Security tests (policy, sandbox escape attempts)
4. Integration tests
5. Offline test (network disabled)
6. Build artifact (executable, bundled model)

Failures block merge. Manual approval required for release builds.

### Rationale
- Automated checks catch regressions early
- Security tests run on every commit (no way to merge untested changes)
- Offline test validates privacy promise before release

---

## Decision 13: Licensing and Compliance

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Project bundles open-source components (Ollama, llama.cpp, FAISS, sentence-transformers, etc.). Must comply with their licenses.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **MIT + acknowledge dependencies** | Permissive; compatible with most; allows commercial use | None for this use case |
| GPL | Strong copyleft; contributes back | Restricts proprietary extensions |
| Proprietary | Full control | Incompatible with open-source foundation |

### Decision
**MIT license for local-ai project**; all bundled dependencies' licenses honored and listed in `LICENSES.txt`.

### Rationale
- MIT is permissive and compatible with GPL-compatible components
- Acknowledges that project stands on shoulders of open-source community
- No legal risk for users building on top

---

## Decision 14: Hardware Target and Constraints

**Status**: DECIDED  
**Date**: 2026-09-04  
**Phase**: 0 (Architecture)

### Context
Local-first means must run on consumer hardware (laptops, desktops). Cannot assume high-end GPU or unlimited RAM.

### Alternatives

| Option | Pros | Cons |
|--------|------|------|
| **Target: mid-range laptop (8GB RAM, CPU); optimize for 4GB** | Runs on most existing hardware | Limits model size; may require quantization |
| Optimize for desktop only (32+ GB RAM) | Better performance | Excludes laptop users |
| Require GPU | Better performance | Not everyone has GPU; adds complexity |

### Decision
**Primary target**: Laptops with 8GB+ RAM and 4-core CPU.  
**Recommended**: 16GB RAM, 8-core CPU, SSD.  
**Model selection**: Quantized models (GGUF int4/int5) for efficiency.

### Rationale
- Most developers have laptops meeting this spec
- Quantized models (4-bit) run acceptably on CPU
- GPU support optional (beneficial but not required)
- SSD requirement is industry standard now

---

## Summary Table

| Decision | Choice | Rationale (One Line) |
|----------|--------|-------------------|
| Desktop UI | Tauri + React | Lightweight; Rust security; web patterns reuse |
| Backend | Rust | Memory safety; security-critical code |
| LLM Runtime | llama.cpp + Ollama abstraction | Offline; flexible; no cloud dependency |
| Database | SQLite + FAISS | Zero setup; ACID; file-based; offline |
| Sandbox | Native OS isolation | Performant; uses existing OS capabilities |
| Build | Cargo + npm/pnpm + pip | Native tooling per language |
| Tests | Four tiers (unit/integration/security/offline) | Comprehensive; catches regressions at each layer |
| Logging | Structured JSON to local files | Observable; private; auditable |
| Embeddings | FAISS + local embedding model | Offline; no cloud dependency |
| Config | TOML files | Human-editable; versionable; hot-reload for policy |
| VCS | Git | Industry standard; easy CI/CD |
| CI/CD | GitHub Actions | Automated; security gates on every commit |
| License | MIT | Permissive; open-source compatible |
| Hardware | 8GB+ laptop; optimize for 4GB | Accessible to most developers |

---

## Review and Revision Process

Decisions are final once documented here and reviewed. Any change requires:
1. Justification (why is the prior decision insufficient?)
2. Reevaluation of alternatives (has landscape changed?)
3. Impact assessment (what breaks if we switch?)
4. Explicit approval (documented in a follow-up decision record)

Decisions touching **security, cloud dependency, or user data** require human approval before implementation.
