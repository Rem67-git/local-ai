# ARCHITECTURE — Local Autonomous AI Platform

## 1. Overview

This document describes the target architecture for a **local-first, privacy-first, offline-first** autonomous AI agent platform. It is a runtime for complex, multi-step missions (analysis, planning, tool use, file manipulation, document search, code execution, observation, correction, verification, memory, resumption, human validation, reporting) entirely on the user's machine.

**Not a chatbot.** This is an **agent runtime** — persistent, autonomous, extensible, capable of complex tasks with human supervision built in.

---

## 2. Core Runtime Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                        USER / MISSION                                │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────────┐
│                     APPLICATION CORE                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Mission Manager · Agent Runtime · Planner · Memory Manager   │  │
│  │ Context Manager · Policy Engine · Event Bus                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────────┬───────────────────────────────────────────┘
                           │
            ┌──────────────┴──────────────┐
            │                             │
      ┌─────▼──────┐            ┌────────▼────────┐
      │ Local LLM  │            │  Tool Router    │
      │ Runtime    │            │  & Dispatcher   │
      └────────────┘            └────────┬────────┘
                                         │
                        ┌────────────────┼────────────────┐
                        │                │                │
                  ┌─────▼────┐  ┌────────▼────┐  ┌───────▼──────┐
                  │Filesystem│  │  Shell /    │  │   Browser    │
                  │ Access   │  │  Code Exec  │  │   Tools      │
                  └──────────┘  └─────────────┘  └──────────────┘
                        │                │                │
                        └────────────────┼────────────────┘
                                         │
                        ┌────────────────▼────────────────┐
                        │      SANDBOX / ISOLATION        │
                        │  (Process, Filesystem, Network) │
                        └────────────────────────────────┘
```

---

## 3. Mandatory Action Pipeline

Every action **must pass through this chain without exception**:

```
LLM Output
   ↓
[1] Structured Action (JSON schema)
   ↓
[2] Schema Validation
   ↓
[3] Policy Engine Check (ALLOW / DENY / ASK_USER / SANDBOX_ONLY)
   ↓
[4] Permission Check (capability matched to user role/budget)
   ↓
[5] Risk Assessment (threat classification)
   ↓
[6] Human Approval (if required)
   ↓
[7] Sandbox Execution (resource limits, isolation)
   ↓
[8] Result Validation
   ↓
[9] Observation (agent sees outcome)
   ↓
Agent Loop
```

No shortcut. No LLM output becomes an operation without the full chain.

---

## 4. Core Components

### 4.1 Mission Manager
- Receives user mission statement
- Creates `MissionState` object with goal, budget (tokens, actions, time), starting context
- Assigns task to appropriate agent (or supervisor)
- Tracks progress, checkpoints, completion criteria
- Persists state to allow resumption after interruption

### 4.2 Agent Runtime
- Implements the agentic loop: **observe → think → plan → act → observe**
- Executes structured tool calls via Tool Router
- Tracks budget (tokens consumed, actions taken, errors, retries)
- Detects loops (repeated actions, stalled progress) and escalates
- Emits events to Event Bus for observation

### 4.3 Planner
- Receives goal from Mission Manager
- Decomposes into sub-goals, creates action DAG (Directed Acyclic Graph)
- Generates strategy (sequential, parallel, branching, conditional)
- Selects tools and execution order
- Adaptive: replans if goals change or strategy fails

### 4.4 Memory Manager
- Maintains **five types of memory** (detailed in DECISIONS.md / phase 7):
  - **Context History** — recent interactions, decisions, errors
  - **Semantic Index** — embeddings for retrieval-augmented generation
  - **Long-term Knowledge** — patterns learned from past missions
  - **Tool Observations** — cumulative tool behavior, failure modes
  - **Checkpoints** — mission resume points, state snapshots
- Implements pruning (old entries removed), recall (relevant context retrieved)
- No cloud. All vectors/indices stored locally (SQLite + FAISS).

### 4.5 Context Manager
- Builds LLM prompt dynamically:
  - System instructions (immutable)
  - User instructions (this session only)
  - Untrusted data (documents, files) — clearly labeled, cannot override policy
  - Retrieved context (memory hits)
  - Tool definitions (available in this state)
  - Current task/goal
  - Budget status
- Manages token budget, truncates if needed

### 4.6 Policy Engine
- **Independent component** (not delegable to LLM)
- Configuration per tool, folder, command, risk level, agent role
- Verdict verdicts: `ALLOW`, `DENY`, `ASK_USER`, `SANDBOX_ONLY`
- Examples:
  - `shell execute in /tmp` → ALLOW
  - `shell execute rm -rf /` → DENY (regardless of LLM request)
  - `read ~/passwords.txt` → ASK_USER
  - `network request` → SANDBOX_ONLY (isolated subprocess, monitored)
- Loaded from configuration file, modifiable by user, not by LLM

### 4.7 Event Bus
- Asynchronous event emitter for:
  - `action_started`, `action_completed`, `action_failed`
  - `mission_checkpoint`, `mission_complete`, `mission_error`
  - `budget_warning`, `budget_exceeded`
  - `loop_detected`, `human_approval_required`
  - `tool_error`, `policy_violation`
- Allows UI to update in real-time, logging to persist events, alerts to trigger

---

## 5. Multi-Agent Pattern

```
┌──────────────────────────────────┐
│      Supervisor Agent            │
│  (Goal routing, delegation)      │
└──────────────────────────────────┘
           │
    ┌──────┼──────┬──────────┬─────────┐
    │      │      │          │         │
┌───▼──┐ ┌─▼──┐ ┌─▼──┐ ┌────▼──┐ ┌──▼──┐
│Planner│ │Code│ │Test│ │Analysis│ │File │
└───────┘ └─────┘ └─────┘ └───────┘ └─────┘
Specialist agents created ON DEMAND, not pre-instantiated.
Each agent declares: role, capabilities, allowed tools, permissions, budget, context.
```

**Rule**: Do not create specialized agents until complexity justifies it. A single general-purpose agent with a role parameter is sufficient initially.

---

## 6. Proposed Directory Structure

```
/local-ai
├── README.md                   # User-facing overview
├── CLAUDE.md                   # Development guide (this file references it)
├── ARCHITECTURE.md             # This file
├── DECISIONS.md                # Technology choices
├── PHASES.md                   # Development roadmap
├── SECURITY.md                 # Policy engine, threat model, sandbox
├── THREAT_MODEL.md             # Threats, mitigations, tests
├── OFFLINE.md                  # Offline guarantees and verification
├── CLAUDE.md                   # (Claude Code development instructions)
│
├── apps/
│   └── desktop/                # Tauri + React frontend
│       ├── src/
│       │   ├── components/      # React UI components
│       │   ├── pages/           # Page layouts
│       │   ├── hooks/           # Custom React hooks
│       │   └── App.tsx          # Root component
│       ├── src-tauri/           # Rust backend for Tauri
│       │   ├── src/main.rs      # Tauri window/event setup
│       │   ├── commands/        # IPC command handlers
│       │   └── Cargo.toml
│       ├── package.json
│       └── tauri.conf.json
│
├── core/                       # Runtime core (Rust)
│   ├── agent/                  # Agent runtime loop
│   │   ├── lib.rs
│   │   ├── executor.rs         # Execute structured actions
│   │   ├── observations.rs     # Observe results
│   │   └── tests/
│   ├── planner/                # Goal decomposition, DAG planning
│   │   ├── lib.rs
│   │   ├── dag.rs
│   │   └── tests/
│   ├── memory/                 # Context history, semantic index
│   │   ├── lib.rs
│   │   ├── storage.rs          # Persist to SQLite
│   │   └── tests/
│   ├── context/                # Build prompts, manage context window
│   │   ├── lib.rs
│   │   ├── truncation.rs       # Token budgeting
│   │   └── tests/
│   ├── policy/                 # Policy engine, verdict logic
│   │   ├── lib.rs
│   │   ├── config.rs           # Load rules from config
│   │   ├── evaluator.rs        # Evaluate action against rules
│   │   └── tests/
│   ├── events/                 # Event bus
│   │   ├── lib.rs
│   │   └── bus.rs
│   ├── mission/                # Mission state, manager
│   │   ├── lib.rs
│   │   ├── state.rs
│   │   └── tests/
│   ├── Cargo.toml
│   └── lib.rs                  # Expose public API
│
├── inference/                  # Local LLM runtime
│   ├── llama.cpp/              # llama.cpp integration
│   │   ├── src/
│   │   ├── build.rs            # C++ binding compilation
│   │   └── Cargo.toml
│   ├── ollama/                 # Ollama HTTP client
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── model-manager/          # Download, cache, select models
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── models/                 # Bundled/downloaded LLM weights
│   │   └── (models stored here, excluded from version control)
│   └── lib.rs
│
├── tools/                      # Built-in tool implementations
│   ├── filesystem/             # File read/write/search in workspace
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── shell/                  # Shell command execution (sandboxed)
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── coding/                 # Code reading, parsing, modification
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── documents/              # Document parsing (PDF, markdown, etc.)
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── rag/                    # Retrieval-augmented generation
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── registry.rs             # Tool catalog and routing
│   └── Cargo.toml
│
├── sandbox/                    # Isolation layer
│   ├── src/
│   │   ├── lib.rs
│   │   ├── subprocess.rs       # Restricted subprocess execution
│   │   ├── filesystem.rs       # Chroot, filesystem restrictions
│   │   ├── network.rs          # Network sandbox (proxy, deny-by-default)
│   │   └── windows.rs          # Windows AppContainer implementation
│   ├── Cargo.toml
│   └── tests/
│
├── database/                   # SQLite schema and migrations
│   ├── schema.sql              # Initial schema
│   ├── migrations/             # Ordered SQL files for schema changes
│   │   ├── 001_init.sql
│   │   ├── 002_memory.sql
│   │   └── ...
│   └── README.md               # Schema documentation
│
├── tests/                      # Test suites (organized by layer)
│   ├── unit/                   # Unit tests for components
│   │   ├── test_policy.rs
│   │   ├── test_planner.rs
│   │   └── ...
│   ├── integration/            # Integration tests (agent + tools + DB)
│   │   ├── test_end_to_end.rs
│   │   └── test_mission_flow.rs
│   ├── security/               # Security-specific tests
│   │   ├── test_path_traversal.rs
│   │   ├── test_unauthorized_access.rs
│   │   ├── test_network_sandbox.rs
│   │   └── test_privilege_escalation.rs
│   ├── agent/                  # Agent behavior tests
│   │   ├── test_loop_detection.rs
│   │   ├── test_hallucination.rs
│   │   └── test_budget.rs
│   ├── offline/                # Offline functionality tests
│   │   ├── test_no_network.rs
│   │   └── test_bundled_models.rs
│   └── fixtures/               # Test data and mocks
│       ├── sample_mission.json
│       ├── malicious_code.py
│       └── ...
│
├── .gitignore
├── Cargo.toml                  # Workspace root
├── Cargo.lock
├── package.json                # (if Node/npm used at root level)
└── .env.example                # Example environment variables
```

---

## 7. State Management

### 7.1 MissionState
```json
{
  "id": "mission_xyz",
  "goal": "Analyze the project and fix errors",
  "status": "in_progress",
  "created_at": "2026-09-04T10:30:00Z",
  "updated_at": "2026-09-04T10:35:45Z",
  "budget": {
    "tokens_allocated": 10000,
    "tokens_consumed": 2450,
    "actions_allocated": 50,
    "actions_consumed": 12,
    "time_limit_seconds": 3600,
    "time_consumed": 325
  },
  "assigned_agent": "planner_v1",
  "sub_tasks": [
    { "id": "task_1", "status": "completed", "result": "..." },
    { "id": "task_2", "status": "in_progress", "result": null }
  ],
  "checkpoints": [
    {
      "id": "checkpoint_1",
      "timestamp": "2026-09-04T10:32:00Z",
      "memory_snapshot": "..."
    }
  ],
  "errors": [],
  "loop_detection": {
    "last_actions": [...],
    "is_looping": false
  }
}
```

### 7.2 Task Status
- `pending` → `in_progress` → `completed` / `failed` / `blocked`
- Each task tracks: input, output, errors, execution time, tool calls

### 7.3 Budget Enforcement
- Tokens: counted per LLM call, mission fails if exceeded
- Actions: counted per tool invocation, mission fails if exceeded
- Time: wall-clock limit, mission interrupted and reported as incomplete if exceeded
- No auto-retry loops past budget

---

## 8. Observable Requirements

### 8.1 `local-ai doctor`
Diagnostic command that verifies:
- LLM runtime is available and responsive
- Models are downloaded and accessible
- Database is initialized and accessible
- Embedding library (FAISS) is functional
- Filesystem permissions are correct
- Sandbox is operational
- Network isolation working
- Policy engine loads without errors

### 8.2 `local-ai offline-test`
Verifies full functionality with network **completely disabled**:
- Mission can be created and started
- LLM can be queried (local inference)
- Tools can execute (filesystem, shell, coding)
- Results can be logged and persisted
- State can be resumed from checkpoint
- Reports successful execution

---

## 9. Data Flow Example: File Analysis Mission

```
User Input:
  "Analyze src/main.py for errors, list them, and propose fixes"

↓ [Mission Manager]
  Creates MissionState { goal, budget, context }

↓ [Planner]
  Decomposes:
    1. Read src/main.py
    2. Parse code structure
    3. Identify errors (linting, type, logic)
    4. Propose fixes

↓ [Agent Loop Iteration 1]
  LLM observes goal + context
  Outputs: { "action": "file_read", "path": "src/main.py" }

↓ [Pipeline: Action → Validation → Policy → Permission → Risk → Approval]
  Policy: ✓ ALLOW (within workspace)
  Execution: Sandbox reads file

↓ [Result]
  File contents returned to agent, added to context

↓ [Agent Loop Iteration 2]
  LLM observes file contents + goal
  Outputs: { "action": "tool_call", "tool": "code_parser", "input": "..." }

↓ [Policy → Sandbox → Result]

... (repeat until sub-goals completed)

↓ [Mission Complete]
  All findings in memory
  Report generated
  State persisted for future reference

↓ [User]
  Receives analysis and fix proposals
```

---

## 10. Offline Guarantees

- **LLM inference**: Local (llama.cpp or Ollama)
- **Memory**: SQLite (file-based)
- **Embeddings**: FAISS (no API calls)
- **Documents**: Loaded and processed locally
- **Configuration**: Local files
- **Logging**: Local files, no remote telemetry

**Network**: Refused by default. Tools that require network (web scraping, API calls) only execute if explicitly enabled by user and sandboxed.

---

## 11. Evolution and Versioning

- **ARCHITECTURE.md** is updated after each major phase completion.
- **DECISIONS.md** records technology choices as they are made.
- **PHASES.md** tracks progress and proof of completion for each of 15 phases.
- No architectural decisions are final until reviewed and committed.
- Any choice touching security, cloud dependency, or user data requires explicit human approval.

---

## 12. Implementation Status (as of Session 2 - Continued)

**Completed Components**:
- Phase 0: Architecture, decisions, roadmap (COMPLETE)
- Phase 1: LLM Runtime trait, Ollama HTTP client, model manager, config system (code complete, build pending)
- Phase 2: Agent runtime with agentic loop, executor, budget tracking, loop detection
- Phase 3: Planner with goal decomposition, DAG with topological sort, strategy selection
- Phase 4: Tool system with registry, tool trait, 4 builtin tools
- Phase 5: Filesystem sandbox with path validator and workspace manager

**Crates Implemented**:
- `core/inference` — LLM runtime abstraction (runtime.rs, ollama.rs, config.rs, models.rs)
- `core/cli` — CLI entry point with commands for model selection, doctor, offline-test
- `core/agent` — Agent loop, executor, action schema, budget, loop detection
- `core/mission` — Mission state, task management, lifecycle
- `core/events` — Event bus with async broadcast
- `core/planner` — Goal struct, DAG with topological sort, strategy enum, plan struct
- `core/tools` — Tool trait, registry, 4 builtin tools with schemas
- `core/sandbox` — PathValidator with traversal prevention, WorkspaceManager

**Test Results**:
- ✅ agent: 10 tests (budget, actions, loop detection, executor)
- ✅ events: 1 test (event bus)
- ✅ inference: 7 tests (config, models, Ollama)
- ✅ mission: 4 tests (state, manager)
- ✅ planner: 6 tests (goal, DAG, topological sort)
- ✅ tools: 4 tests (registry, tool execution)
- ✅ sandbox: 8 tests (path validation, workspace boundaries)
- **Total: 40 tests passing**

**Next Phases**:
- Phase 4: Tool System (tool trait, registry, built-in tools)
- Phase 5: Filesystem Sandbox (path validation, workspace boundary)
- Phase 6: Coding Agent (code reading, error detection, fixes)
- ...and 9 more phases to release

---

## 13. Next Steps

This architecture is the target. Phase 0 is complete when:
1. This document is finalized
2. DECISIONS.md documents technology choices
3. PHASES.md outlines 15 phases
4. Repository structure is ready for Phase 1 implementation

Phase 1 begins with local LLM runtime setup (llama.cpp / Ollama integration).
