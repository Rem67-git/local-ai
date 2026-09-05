# PHASES — Development Roadmap and Progress Tracking

This document defines the 15 development phases for the Local Autonomous AI platform. Each phase has:
- **Goal**: what should be done
- **Status**: PENDING | IN_PROGRESS | BLOCKED | COMPLETE
- **Proof of Completion**: checklist from CLAUDE.md section 8

---

## Phase 0: Architecture & Documentation ✏️

**Goal**: Establish architecture, technology decisions, and development roadmap. Create foundational documentation.

**Deliverables**:
- [x] ARCHITECTURE.md — target architecture, component diagram, directory structure, data flow
- [x] DECISIONS.md — technology choices with alternatives and rationale
- [x] PHASES.md (this file) — 15 phases defined, initial status PENDING
- [x] README.md — user-facing overview, quick start guide
- [x] .gitignore — exclude models, build artifacts, user data, secrets
- [x] Git repo initialized

**Status**: COMPLETE  
**Proof of Completion**:
- [x] All four docs exist and are readable
- [x] Architecture does not contradict CLAUDE.md
- [x] Technology choices are justified
- [x] 15 phases listed and understood
- [x] README explains project purpose to new developers
- [x] Git repo initialized with clean history

**Estimated Duration**: 1 session

---

## Phase 1: Local LLM Runtime 🧠

**Goal**: Integrate local LLM inference. User can select and run a model without cloud API calls.

**Deliverables**:
- [x] llama.cpp Rust bindings (or HTTP wrapper) — llama.rs stub
- [x] Ollama HTTP client — full implementation
- [x] Abstraction layer (`LLMRuntime` trait) — runtime.rs with health checks
- [x] Model download manager (auto-fetch model from HuggingFace or similar) — models.rs
- [x] Model caching (avoid re-downloading) — config-based with ~/.local-ai/models/
- [x] Configuration (select default model, set context window, temperature, etc.) — config.rs (TOML)
- [x] CLI command: `local-ai list-models`, `local-ai select-model <model_id>` — commands.rs
- [ ] Integration tests for LLM inference — tests to execute after build

**Status**: IN_PROGRESS  
**Build Status**: Code complete, awaiting MSVC linker installation (VS Build Tools in progress)  
**Proof of Completion**:
- [ ] Code implemented (model loader, inference wrapper, config)
- [ ] Tests executed: unit tests pass, inference returns token sequences
- [ ] Manual test: user runs `local-ai select-model` and downloads a model (e.g., Mistral 7B)
- [ ] Offline verification: `local-ai offline-test` runs inference without network
- [ ] Docs updated (ARCHITECTURE.md notes which models supported)

**Dependencies**: None (Phase 0 must be complete)  
**Estimated Duration**: 2 sessions

---

## Phase 2: Agent Runtime 🤖

**Goal**: Core agentic loop: observe, think, plan, act. Agent can execute structured tool calls.

**Deliverables**:
- [x] Agent struct (state, context, budget) — runtime.rs
- [x] Observation system (LLM receives current state + context) — build_context() in runtime
- [x] Action generation (LLM outputs structured JSON) — StructuredAction.from_json()
- [x] Action executor (routes to tool, captures result) — executor.rs
- [x] Budget tracking (tokens, actions, time limits) — budget.rs
- [x] Loop detection (repeated actions, stalled progress) — loop_detection.rs
- [x] Error handling (parse errors, tool failures, timeouts) — comprehensive error types
- [x] Unit and integration tests for agent loop — tests in each module
- [ ] Integration test: end-to-end mission execution (LLM→action→execution→observation)
- [ ] `local-ai doctor` command update (verify agent runtime is responsive)

**Status**: IN_PROGRESS  
**Implementation**: 3 crates (agent, mission, events) with core loop, state management, and event bus  
**Proof of Completion**:
- [ ] Code implemented (agent loop, executor, budget)
- [ ] Tests executed: loop runs, produces actions, catches errors
- [ ] Manual test: user provides simple goal ("List files in current dir"), agent runs 3-5 iterations and completes
- [ ] `local-ai doctor` runs and reports "agent runtime OK"
- [ ] ARCHITECTURE.md updated with loop details, budget enforcement

**Dependencies**: Phase 1 (LLM must respond)  
**Estimated Duration**: 2 sessions

---

## Phase 3: Planner 📋

**Goal**: Goal decomposition and action planning. Agent can tackle multi-step missions.

**Deliverables**:
- [x] Planner struct (goal, sub-goals, DAG) — lib.rs
- [x] Goal struct with decomposition logic — goal.rs (GoalStatus enum: pending/in_progress/completed/failed/blocked)
- [x] DAG construction (dependencies, parallelization points) — dag.rs (topological_sort, next_executable, validate_acyclic)
- [x] Strategy selection (sequential, parallel, conditional) — strategy.rs (Strategy enum with variants)
- [x] Replanning logic (if a goal fails, adjust strategy) — replanner.rs (Replanner stub)
- [x] Plan struct with execution order — plan.rs (mission_goal, sub_goals DAG, strategy, execution_order)
- [ ] Unit and integration tests for planning
- [ ] Example mission: "Analyze project structure and list errors"
- [ ] LLM-based goal decomposition (currently stub)

**Status**: IN_PROGRESS  
**Code Implementation**: Core planner crate complete (goal.rs, dag.rs, strategy.rs, plan.rs, replanner.rs)  
**Proof of Completion**:
- [x] Code implemented (Planner struct, Goal struct, DAG with topological sort, Plan struct, Strategy enum)
- [ ] Tests executed: planner decomposes goal, produces valid action sequences
- [ ] Manual test: mission "Analyze ./src" runs, planner creates 4-5 sub-goals, executes in order
- [ ] Integration test with Phase 2: agent runs planner's goals
- [ ] ARCHITECTURE.md updated with planner details

**Dependencies**: Phase 2 (agent must execute actions)  
**Estimated Duration**: 2 sessions

---

## Phase 4: Tool System 🔧

**Goal**: Build tool catalog. Agent can invoke filesystem, shell, code parsing, etc.

**Deliverables**:
- [x] Tool trait (name, input schema, output schema, execution function) — tool.rs
- [x] Tool registry (catalog of available tools) — registry.rs
- [x] Tool router (matches action to tool) — registry.execute()
- [x] Built-in tools:
  - [x] `file_read` — read file contents (mock)
  - [x] `file_write` — write to file (mock)
  - [x] `file_list` — list directory contents (mock)
  - [x] `shell_exec` — run shell command (mock)
- [x] JSON schema validation for tool inputs (input_schema method)
- [x] Error messages for invalid inputs (ToolError enum)
- [x] Unit tests for each tool (4 tests passing)
- [ ] Integration tests (agent → tool → result) — next phase
- [ ] code_parse and code_modify tools (deferred to Phase 6)

**Status**: IN_PROGRESS  
**Implementation**: Tool trait with async execute, ToolRegistry with register/execute/list, 4 builtin tools with schemas  
**Proof of Completion**:
- [x] Code implemented (tool trait, registry, builtin tools)
- [x] Tests executed: 4 tests passing (register, list, execute, error handling)
- [ ] Manual test: agent calls file_read, file_list via registry
- [x] ARCHITECTURE.md updated with tool catalog

**Dependencies**: Phase 2 (agent routes to tools)  
**Estimated Duration**: 2 sessions

---

## Phase 5: Filesystem Sandbox 🔒

**Goal**: Restrict filesystem access. Agent cannot escape workspace or access arbitrary files.

**Deliverables**:
- [x] Path validation (normalize, check against workspace root) — validator.rs
- [x] Workspace boundary enforcement (all operations confined to workspace) — workspace.rs
- [x] Symbolic link handling (prevent escapes via component analysis) — validator.rs
- [ ] Permission checking (read-only for system files, read-write for workspace) — Phase 6
- [ ] Windows AppContainer setup (advanced isolation) — Phase 11
- [ ] Unix chroot/seccomp setup (advanced isolation) — Phase 11
- [x] Security tests (path traversal attempts, escape attempts) — 8 tests
- [ ] Fallback to restricted subprocess if native sandbox unavailable

**Status**: IN_PROGRESS  
**Implementation**: PathValidator with normalization, WorkspaceManager with boundary enforcement  
**Proof of Completion**:
- [x] Code implemented (path validator, workspace manager)
- [x] Security tests executed: 8 tests (path traversal blocked, symlink escape blocked, boundary enforced)
- [ ] Manual test: attempt `file_read ../../../etc/passwd` → rejected
- [ ] `local-ai offline-test` includes sandbox verification
- [ ] SECURITY.md updated with sandbox guarantee

**Dependencies**: Phase 4 (tools that perform filesystem operations)  
**Estimated Duration**: 2 sessions

---

## Phase 6: Coding Agent 🖊️

**Goal**: Agent can read, analyze, and modify code. Can detect and fix basic errors.

**Deliverables**:
- [x] Code parser (syntax validation, language detection) — parser.rs
- [x] Error detection (language-specific, linting issues) — analyzer.rs
- [x] Code modification (replace, insert, delete, bulk edits) — modifier.rs
- [ ] Test running (execute unit tests, parse results) — deferred to Phase 6b
- [ ] Diff generation (show what changed) — can be added to modifier
- [x] Integration with code tools from Phase 4 (CodeAnalyzer returns issues)
- [ ] Example mission: "Fix syntax errors in main.py"
- [x] Tests for parsing, editing, analyzing (14 tests)

**Status**: IN_PROGRESS  
**Implementation**: CodeParser with 12+ language support, CodeAnalyzer with language-specific rules, CodeModifier with safe transformations  
**Proof of Completion**:
- [x] Code implemented (parser, analyzer, modifier)
- [x] Tests executed: 14 tests (parsing, analysis, modifications)
- [ ] Manual test: mission "Analyze src/main.py and list errors" succeeds
- [ ] Manual test: mission "Fix the broken test in test_utils.py" modifies file and re-runs tests
- [ ] ARCHITECTURE.md updated with coding agent details

**Dependencies**: Phase 5 (sandbox must be enforced)  
**Estimated Duration**: 2 sessions

---

## Phase 7: Memory System 💾

**Goal**: Agent can remember past interactions, retrieve relevant context, and learn patterns.

**Deliverables**:
- [x] Memory types:
  - [x] Context history (recent decisions, errors, results) — memory_entry.rs
  - [x] ToolObservation (cumulative tool error modes) — memory_entry.rs
  - [x] DecisionRecord (past decisions) — memory_entry.rs
  - [x] LearningPattern (patterns discovered) — memory_entry.rs
  - [x] MissionCheckpoint (mission resume points) — memory_entry.rs
- [ ] Embedding model selection and download — deferred to Phase 8
- [ ] Vector store setup (FAISS) — deferred to Phase 8
- [x] Recall logic (retrieve similar past contexts) — recall.rs
- [x] Pruning (remove stale entries) — context_history.rs
- [ ] Persistence (SQLite schema for memory) — deferred to integration
- [x] Tests for storage, recall, pruning (12 tests)

**Status**: IN_PROGRESS  
**Implementation**: MemoryEntry storage, ContextHistory with limits, RecallEngine with similarity matching  
**Proof of Completion**:
- [x] Code implemented (memory types, storage, recall)
- [x] Tests executed: 12 tests (entry lifecycle, history, pruning, recall ranking)
- [ ] Manual test: agent recalls similar past missions and applies learned strategies
- [ ] Offline test: memory persists without network
- [x] Basic recall engine with Jaccard similarity

**Dependencies**: Phase 4 (tools generate memories), Phase 5 (storage)  
**Estimated Duration**: 2 sessions

---

## Phase 8: Retrieval-Augmented Generation (RAG) 📚

**Goal**: Agent can ingest and search documents. Augment LLM context with relevant information.

**Deliverables**:
- [x] Document ingestion (text, markdown, code, JSON, HTML) — document.rs
- [x] Chunking (break docs into 500-char segments with 100-char overlap) — document.rs
- [x] Word-based indexing (inverted index for keyword search) — indexer.rs
- [x] Keyword search (retrieve matching documents) — retriever.rs
- [x] Context augmentation (get_context() for LLM prompt injection) — retriever.rs
- [x] Document parser (6 document types with metadata) — document.rs
- [x] Tests for ingestion, search, retrieval (12 tests)
- [ ] Example mission: "Analyze the requirements in docs/ and compare with code"
- [ ] FAISS embeddings (deferred to Phase 8b for vector similarity)

**Status**: IN_PROGRESS  
**Implementation**: Document with chunking, DocumentIndexer with word indexing, DocumentRetriever with keyword search  
**Proof of Completion**:
- [x] Code implemented (document processor, indexer, retriever)
- [x] Tests executed: 12 tests (document, indexing, retrieval)
- [ ] Manual test: mission ingests README + DESIGN.md, uses them to answer questions
- [ ] Offline test: RAG works without network
- [x] Basic retrieval pipeline working (keyword → documents → context)

**Dependencies**: Phase 7 (embeddings system)  
**Estimated Duration**: 2 sessions

---

## Phase 9: User Interface 🖥️

**Goal**: Desktop UI for missions, logs, and real-time feedback. User can launch missions and monitor progress.

**Deliverables**:
- [x] Tauri window setup (1200x900 main window) — tauri.conf.json
- [x] React components:
  - [x] Mission input form (goal textarea) — App.tsx
  - [x] Mission status display (status indicator) — App.tsx
  - [x] Logs viewer (real-time structured logs) — App.tsx
  - [x] Recent missions panel (list of missions) — App.tsx
  - [ ] Memory/history inspector (past missions) — deferred to Phase 9b
  - [ ] Settings panel (model selection) — deferred to Phase 9b
- [x] Real-time updates (log streaming, status display) — foundation in App.tsx
- [x] IPC from React to Rust backend (Tauri commands) — main.rs
- [x] Styling and responsive layout (CSS Grid) — App.css
- [ ] Unit tests for components
- [ ] Manual testing on Windows 11

**Status**: IN_PROGRESS  
**Implementation**: Tauri main window with React frontend, IPC handlers, responsive UI layout  
**Proof of Completion**:
- [x] Code implemented (Tauri + React structure complete)
- [ ] Tests executed: components render, IPC works
- [ ] Manual test: user launches mission via UI, sees live logs and progress
- [ ] UI works offline (no external CDNs, all assets bundled)
- [x] Basic UI skeleton ready for backend integration

**Dependencies**: Phase 6 (agent must be complete for end-to-end demo)  
**Estimated Duration**: 2 sessions

---

## Phase 10: Multi-Agent Supervision 👥

**Goal**: Multiple specialized agents (Planner, Coder, Tester, Analyst, Reviewer) coordinated by Supervisor.

**Deliverables**:
- [ ] Supervisor agent (chooses which specialist to delegate to)
- [ ] Specialist agent templates (each with own tools and role)
- [ ] Delegation protocol (supervisor → specialist → supervisor)
- [ ] Role-based tool access (coder gets code tools, analyst gets doc tools)
- [ ] Communication channel (specialists can query each other)
- [ ] Integration tests (multi-agent workflows)
- [ ] Example mission: "Complex project analysis" → Analyst + Coder + Tester + Reviewer

**Status**: PENDING  
**Proof of Completion**:
- [ ] Code implemented (supervisor, specialist agents, delegation)
- [ ] Tests executed: supervisor routes correctly, specialists complete tasks
- [ ] Manual test: complex mission that benefits from multiple specialists succeeds
- [ ] ARCHITECTURE.md updated with multi-agent pattern

**Dependencies**: Phases 1-9 (all components must be mature)  
**Estimated Duration**: 2 sessions

---

## Phase 11: Security Hardening 🔐

**Goal**: Threat model implementation, permission enforcement, loop detection, and security testing.

**Deliverables**:
- [ ] Threat model (from THREAT_MODEL.md):
  - [ ] Prompt injection (docs/code with false instructions)
  - [ ] Malicious tool output
  - [ ] Path traversal and filesystem escape
  - [ ] Network exfiltration
  - [ ] Resource exhaustion
  - [ ] Privilege escalation
  - [ ] Agent loops and hallucinations
- [ ] Security tests for each threat (in `tests/security/`)
- [ ] Policy engine hardening (edge cases, denial rules)
- [ ] Loop detection tuning (sensitivity, escape strategies)
- [ ] Budget enforcement (no agent runs forever)
- [ ] Permission model (user controls agent autonomy level)
- [ ] Audit logging (detailed action traces for review)

**Status**: PENDING  
**Proof of Completion**:
- [ ] Security tests executed: all threats from THREAT_MODEL.md tested
- [ ] Manual verification: attempt each threat type, all blocked
- [ ] Audit logs reviewed for completeness
- [ ] THREAT_MODEL.md and SECURITY.md updated with test results

**Dependencies**: Phases 1-10 (all prior phases must be complete)  
**Estimated Duration**: 2 sessions

---

## Phase 12: Offline Packaging 📦

**Goal**: Distribute self-contained executable bundle with LLM, models, database schema, no external dependencies.

**Deliverables**:
- [ ] Build script (compile Rust, bundle React, download models)
- [ ] Model bundling (include quantized model in distribution)
- [ ] Database bundling (SQLite schema pre-initialized)
- [ ] Dependency vendoring (all Rust/Node deps included)
- [ ] Executable packaging (native binary on Windows)
- [ ] Installation guide (first-run setup)
- [ ] `local-ai doctor` verification
- [ ] `local-ai offline-test` proof of offline capability
- [ ] Release checklist

**Status**: PENDING  
**Proof of Completion**:
- [ ] Build script works end-to-end
- [ ] Standalone executable runs on Windows 11 (no pre-installed runtimes)
- [ ] `local-ai offline-test` succeeds with network disabled
- [ ] All components verified offline
- [ ] Installer created and tested

**Dependencies**: Phases 1-11  
**Estimated Duration**: 1 session

---

## Phase 13: Performance Optimization ⚡

**Goal**: Profile, optimize, and scale. Agent responds quickly, handles large missions efficiently.

**Deliverables**:
- [ ] Profiling (identify bottlenecks: LLM latency, tool overhead, memory)
- [ ] Token optimization (shorter prompts without losing context)
- [ ] Tool caching (memoize common operations)
- [ ] Parallel tool execution (where possible)
- [ ] Database indexing (fast memory queries)
- [ ] Large-scale testing (100-task missions)
- [ ] Benchmarks (latency, throughput, memory usage)

**Status**: PENDING  
**Proof of Completion**:
- [ ] Profiling data collected and analyzed
- [ ] Bottlenecks identified and addressed
- [ ] Benchmarks show improvement (before/after)
- [ ] Large-scale test (100-task mission) completes in reasonable time
- [ ] DECISIONS.md updated with performance-related choices

**Dependencies**: Phases 1-12  
**Estimated Duration**: 1 session

---

## Phase 14: Release 🚀

**Goal**: Documentation, packaging, distribution, user support.

**Deliverables**:
- [ ] README.md — comprehensive user guide
- [ ] INSTALLATION.md — platform-specific install instructions
- [ ] USAGE.md — mission definition, examples, troubleshooting
- [ ] API.md — developer documentation (if extending with plugins)
- [ ] LICENSE.txt and LICENSES.txt (attribution for all dependencies)
- [ ] Changelog — version history
- [ ] Release binary (Windows exe, + Mac/.dmg and Linux/.AppImage if applicable)
- [ ] Announcement (blog post, social, etc.)
- [ ] Version number (semantic versioning)

**Status**: PENDING  
**Proof of Completion**:
- [ ] All documentation written and reviewed
- [ ] Release binary tested on target platforms
- [ ] Install, run, and complete a mission from scratch (fresh user simulation)
- [ ] Support channels ready (issue tracker, email, etc.)

**Dependencies**: Phases 1-13  
**Estimated Duration**: 1 session

---

## Proof of Completion Template

Each phase must verify:

✅ **Code**: Implementation complete, code is readable and modular  
✅ **Tests**: Tests executed in this session, output pasted or summarized  
✅ **Security**: Relevant threat model tests run (or documented why N/A)  
✅ **Offline**: `local-ai offline-test` passes (or documented why N/A)  
✅ **Docs**: ARCHITECTURE.md, SECURITY.md, DECISIONS.md, PHASES.md updated  

**Never** claim a phase is complete without evidence. If a verification couldn't be done (tool missing, env incompatible), state explicitly rather than omit.

---

## Phase Status Summary

| Phase | Title | Status | Est. Duration |
|-------|-------|--------|----------------|
| 0 | Architecture | COMPLETE | 1 session |
| 1 | Local LLM | IN_PROGRESS | 2 sessions |
| 2 | Agent Runtime | IN_PROGRESS | 2 sessions |
| 3 | Planner | IN_PROGRESS | 2 sessions |
| 4 | Tool System | IN_PROGRESS | 2 sessions |
| 5 | Filesystem Sandbox | IN_PROGRESS | 2 sessions |
| 6 | Coding Agent | IN_PROGRESS | 2 sessions |
| 7 | Memory System | IN_PROGRESS | 2 sessions |
| 8 | RAG | IN_PROGRESS | 2 sessions |
| 9 | UI | IN_PROGRESS | 2 sessions |
| 10 | Multi-Agent | PENDING | 2 sessions |
| 11 | Security | PENDING | 2 sessions |
| 12 | Packaging | PENDING | 1 session |
| 13 | Performance | PENDING | 1 session |
| 14 | Release | PENDING | 1 session |

**Total estimated duration**: ~30 sessions (assuming 8-hour days, ~4 weeks)

---

## Progress Notes

- Phase 0 completion is **blocking** — all decisions must be finalized before Phase 1 begins.
- Phases 1-9 form the MVP (end-to-end mission execution).
- Phases 10-11 add sophistication (multi-agent, hardening).
- Phases 12-14 are production readiness (packaging, performance, release).
- Any phase can be marked **BLOCKED** if dependencies aren't met or blockers arise.
- Blocked phases are unblocked by fixing the root cause (usually a prior phase delay).
