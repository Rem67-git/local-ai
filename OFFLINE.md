# Offline Guarantees - Local Autonomous AI

This document specifies the offline-first commitment and verification procedures.

---

## Offline-First Commitment

**The Local Autonomous AI platform operates 100% offline after installation.** No network access is required to run missions, manage memory, or perform any core functionality.

### What Works Offline

✅ **LLM Inference**
- Bundled GGUF quantized model
- Local LLM runtime (Ollama or llama.cpp)
- No external API calls

✅ **Agent Execution**
- Complete agentic loop (observe → think → plan → act)
- All tools (filesystem, shell, code, documents)
- Mission planning and execution

✅ **Memory & Context**
- Context history storage (SQLite)
- Semantic recall (local embeddings)
- Pattern learning

✅ **RAG & Document Processing**
- Document indexing (inverted index)
- Keyword search and retrieval
- Local embeddings (FAISS)

✅ **Desktop UI**
- Full UI functionality
- Mission dashboard
- Real-time logs and status
- Historical mission data

✅ **Configuration & Logs**
- Configuration file parsing (~/.local-ai/config.toml)
- Structured logging (JSON files)
- Audit logging
- Mission archives

### What Requires Network (Opt-In Only)

❌ **Explicitly Disabled by Default**:
- Telemetry (never enabled without user opt-in)
- Cloud sync (not implemented)
- Model downloads (user initiated via CLI)
- External APIs (not integrated)

**If a network feature is ever added**:
1. It is opt-in (default: disabled)
2. User must explicitly enable
3. Clearly documented
4. Audit logged
5. Can be disabled without breaking core functionality

---

## Offline Verification

### Method 1: System Check

```bash
./local-ai doctor
```

Output:
```
✅ Offline System Check

LLM Runtime:
  Status: ✅ Ready (Ollama running locally)
  Model: Mistral 7B (quantized)
  Size: 5.2 GB

Database:
  Status: ✅ SQLite ready
  Path: ~/.local-ai/database.db
  Missions: 12 records

Memory:
  Status: ✅ Ready
  Entries: 156
  Indexes: 10,234 words

Network:
  Status: ✅ DISABLED (by default)
  Policy: DENY all external
  Connected: false

System Readiness: ✅ 100% offline-capable
```

### Method 2: Offline Test

```bash
# Disconnect network (disable WiFi, unplug Ethernet)
./local-ai offline-test
```

Procedure:
1. Verify network is disconnected
2. List available models: `./local-ai list-models` (local only)
3. Run sample mission: `./local-ai run-mission "Test offline"`
4. Check mission completed successfully
5. Verify all logs saved locally

Expected output:
```
✅ Offline Test Passed

Network Status: DISCONNECTED (verified)

Tests:
  1. List models (local)          ✅ PASS
  2. Run mission (sample code)    ✅ PASS (2.3 seconds)
  3. Save mission results         ✅ PASS
  4. Query mission history        ✅ PASS

All operations completed without network.
Offline capability: ✅ VERIFIED
```

### Method 3: Network Trace

```bash
# On Linux/Mac with tcpdump
sudo tcpdump -n "host 192.168.1.100 and (port 80 or port 443)"

# Run mission with network monitor
./local-ai run-mission "Analyze workspace"

# Expected: No packets captured (zero network activity)
```

### Method 4: Process Inspection

```bash
# Check running processes for network connections
lsof -i -P -n | grep local-ai
# Expected: No entries (no network connections)

# Check open network sockets
netstat -tlnp | grep local-ai
# Expected: No entries (no listening sockets)
```

---

## Offline Components

### 1. LLM Runtime (Local)

**Ollama**: Self-hosted LLM server
- Runs on `http://localhost:11434` (local only)
- No internet connectivity required
- All inference local

**Alternative**: llama.cpp (lighter weight)
- Can run without Ollama
- Uses same local models (GGUF format)
- HTTP wrapper on localhost

**Model Storage**: ~/.local-ai/models/
- Quantized models (Q4, Q3, Q2)
- No model downloads during missions
- Model selection at setup time

### 2. Database (Local)

**SQLite**: File-based database
- Location: ~/.local-ai/database.db
- Zero setup required
- All data stored locally
- No cloud sync

**Schemas**:
- missions (mission history)
- tasks (task logs)
- memory (memory entries)
- documents (ingested documents)
- agents (agent configurations)

### 3. Memory & RAG (Local)

**Inverted Index**: Word-based indexing
- Built from local documents
- No external embeddings service
- Keyword search in-process

**Context History**: SQLite storage
- Observations logged locally
- Retrievable without network
- Pruned locally over time

### 4. Desktop App (Local)

**Tauri**: Native desktop app
- No cloud backend
- All UI data in memory
- Logs written to disk
- Fully offline capable

**Backend**: Rust runtime
- Local IPC (inter-process communication)
- No outbound network calls
- Event bus broadcast (local)

### 5. Tools (Sandboxed)

**File Operations**: Workspace-local
- No cloud storage required
- No sync dependencies
- Local filesystem access

**Shell Execution**: Subprocess isolation
- No external commands
- All operations local
- Results stored locally

**Code Analysis**: Local parsing
- No external AST services
- Built-in parser
- Results cached locally

---

## Installation for Offline Operation

### Prerequisite: Download Model

Before going offline, download the LLM model:

```bash
# Download Mistral 7B Q4 (5.2 GB)
./local-ai select-model mistral-7b-q4

# Verify download completed
./local-ai list-models
# Output: mistral-7b-q4 (cached, 5.2 GB)
```

### Setup

```bash
# Build from source or extract release
./scripts/build-release.sh

# Initialize database and configuration
./local-ai doctor

# Verify offline operation
./local-ai offline-test
```

### First Mission

```bash
# All offline (after model downloaded)
./local-ai run-mission "Analyze project structure"
```

---

## Offline Usage Patterns

### Pattern 1: Batch Processing

Download data, disconnect, process offline:

```bash
# 1. Online: Download documents
./local-ai ingest-documents /path/to/documents

# 2. Offline: Process
./local-ai run-mission "Summarize documents"
./local-ai run-mission "Find patterns"
./local-ai run-mission "Generate report"

# 3. Back online: Share results (optional, not required)
```

### Pattern 2: Daily Analysis

```bash
# Night: Run complex analysis (no interruptions)
./local-ai run-mission "Deep code review" --autonomy=autonomous

# Morning: View results and reports
open ~/.local-ai/missions/<mission_id>/report.md
```

### Pattern 3: Multi-User Workspace

```bash
# User A: Private workspace
export WORKSPACE=/home/userA/workspace
./local-ai run-mission "Analyze my code"

# User B: Different workspace
export WORKSPACE=/home/userB/workspace
./local-ai run-mission "Analyze my code"

# All offline, no cross-user interference
```

---

## Offline Guarantees

### Network Policy

```
DEFAULT: DENY
Network can only be enabled:
  1. By explicit user opt-in
  2. In configuration file
  3. Documented in release notes
  4. Reversible (turn off)
  5. Logged in audit
```

### Data Residency

```
ALL DATA STORED LOCALLY:
  ✓ LLM model weights: ~/.local-ai/models/
  ✓ Database: ~/.local-ai/database.db
  ✓ Configuration: ~/.local-ai/config.toml
  ✓ Logs: ~/.local-ai/logs/
  ✓ Missions: ~/.local-ai/missions/
  ✓ Memory: SQLite (local)
  ✓ Audit: ~/.local-ai/audit.log
  
NO DATA SENT ANYWHERE:
  ✗ No telemetry
  ✗ No analytics
  ✗ No error reporting
  ✗ No cloud backup
  ✗ No model training
```

### Dependencies

```
ZERO EXTERNAL DEPENDENCIES:
  ✓ LLM: Local (Ollama/llama.cpp)
  ✓ Database: SQLite (bundled)
  ✓ Embeddings: FAISS (local)
  ✓ UI: Tauri (local)
  ✓ Runtime: Rust/Node (local)
  
NO CLOUD REQUIRED:
  ✗ OpenAI
  ✗ Anthropic
  ✗ Google
  ✗ Azure
  ✗ AWS
  ✗ Any API provider
```

---

## Verification Checklist

### Before Release

- [ ] All missions complete successfully offline
- [ ] `./local-ai doctor` passes with network disabled
- [ ] `./local-ai offline-test` succeeds (network unplugged)
- [ ] Network monitoring (tcpdump) shows zero external traffic
- [ ] All data stored in ~/.local-ai/ directory
- [ ] No hardcoded API endpoints in code
- [ ] No external URLs in default configuration
- [ ] Audit logs complete and local

### Ongoing

- [ ] Monthly: Run offline-test with network disabled
- [ ] Before each release: Verify offline capability
- [ ] Code review: Flag any network I/O
- [ ] Dependency audit: No new cloud dependencies

---

## Troubleshooting Offline Operation

### Problem: "Model not found"

```bash
./local-ai offline-test
# Error: Model mistral-7b-q4 not found

# Solution: Download model while online
./local-ai select-model mistral-7b-q4

# Verify download
ls -lh ~/.local-ai/models/mistral-7b-q4.gguf
# Should be ~5.2 GB
```

### Problem: "Network error during inference"

```
# This should never happen
# If it does, inspect logs:
tail ~/.local-ai/logs/agent.log

# Check network policy:
cat ~/.local-ai/config.toml | grep network

# Verify Ollama running locally:
curl http://localhost:11434/api/tags
```

### Problem: "Mission takes too long offline"

```
# Normal: 2-5 minutes for complex mission
# If > 15 minutes:

# 1. Check memory:
free -h  # Linux
vm_stat  # Mac

# 2. Check CPU:
top  # Linux/Mac

# 3. Reduce mission scope
./local-ai run-mission "Simpler task"

# 4. Check model performance:
./local-ai list-models
# Q4 (slower but better quality)
# Q3 (faster, lower quality)
# Q2 (fastest, poorest quality)
```

---

## Offline Deployment

### Corporate/Offline Environments

For deployment where NO internet access exists:

```bash
# 1. Build on connected machine
./scripts/build-release.sh

# 2. Copy to offline location
cp -r dist/ /offline-location/local-ai

# 3. Configure
export WORKSPACE=/offline-location/workspace
./local-ai doctor

# 4. Run missions
./local-ai run-mission "Analyze codebase"
```

### Multiple Machines (No Cloud Sync)

Each machine is independent:

```bash
# Machine A: Install and configure
./local-ai doctor
./local-ai select-model mistral-7b-q4

# Machine B: Install and configure independently
./local-ai doctor
./local-ai select-model mistral-7b-q4

# Each machine has its own:
# - Models
# - Database
# - Memory
# - Configuration

# To share results: Manual file copy
# (No automatic sync)
```

---

## Future: Optional Cloud Features (Phase 15+)

If cloud features are ever added in future versions:

1. **Always Opt-In**: Default disabled
2. **Clearly Documented**: In CHANGELOG and README
3. **Reversible**: Can be disabled without breaking core
4. **Logged**: All cloud operations in audit log
5. **Configurable**: User controls what syncs

**Example** (hypothetical, not implemented):
```toml
[cloud]
enabled = false  # Default: off
endpoint = "https://cloud.local-ai.example.com"
sync_missions = false
sync_memory = false
# Everything remains local by default
```

---

## Offline Guarantee SLA

**We guarantee**:
- ✅ Core functionality works 100% offline
- ✅ No required cloud dependencies
- ✅ No internet access needed for basic operation
- ✅ All data stored locally
- ✅ No telemetry or tracking

**If you discover an offline operation failure**:
1. File an issue describing the problem
2. We will investigate and fix
3. Priority: Critical (blocks offline operation)

---

**Last Updated**: 2026-09-05  
**Status**: Offline-First Verified  
**Guarantee**: 100% offline operation after installation  
**Review Cycle**: Before each major release
