# Quick Start Guide - Local Autonomous AI

Get up and running in 5 minutes.

---

## Installation (2 minutes)

### Option 1: Pre-built Binary (Recommended)

```bash
# Download latest release
wget https://github.com/local-ai/releases/download/v1.0.0/local-ai-v1.0.0-windows.exe

# Run installer
./local-ai-v1.0.0-windows.exe

# Verify setup
./local-ai doctor
```

### Option 2: Build from Source

```bash
# Clone repository
git clone https://github.com/local-ai/local-ai.git
cd local-ai

# Build
./scripts/build-release.sh

# Verify
./local-ai doctor
```

---

## First Mission (3 minutes)

### Example 1: Analyze a Project

```bash
# Set workspace
export WORKSPACE=/path/to/your/project

# Run mission
./local-ai run-mission "Analyze project structure and list all Python files"
```

**What happens**:
1. Agent reads your project
2. Finds Python files
3. Analyzes them for structure
4. Reports findings

**Output**: Mission results saved to `~/.local-ai/missions/<mission_id>/`

### Example 2: Find Code Issues

```bash
./local-ai run-mission "Find all syntax errors and code quality issues in src/"
```

**Agent will**:
1. Parse all code files
2. Check syntax
3. Identify issues
4. Suggest fixes
5. Report in JSON

### Example 3: Desktop UI

```bash
# Launch desktop app
./local-ai-ui
```

Then:
1. Enter mission goal (e.g., "Analyze workspace")
2. Select workspace path
3. Choose autonomy level (Assisted = default)
4. Click "Start Mission"
5. Watch real-time logs

---

## Common Tasks

### Task 1: Analyze Code Quality

```bash
./local-ai run-mission "Analyze code quality in src/. Check for:
- Unused imports
- Missing error handling
- Code style violations
- Security issues
Report findings in JSON"
```

### Task 2: Generate Documentation

```bash
./local-ai run-mission "Read all Python files in src/ and generate:
1. Module overview
2. Key functions
3. Dependencies
4. Usage examples"
```

### Task 3: Run Tests & Report

```bash
./local-ai run-mission "
1. Find all test files
2. Run pytest
3. Parse results
4. Generate coverage report
5. Suggest missing tests"
```

### Task 4: Fix Errors

```bash
./local-ai run-mission "
1. Identify all compilation errors
2. Suggest fixes (explain why)
3. Propose patches
4. Do NOT apply without approval"
```

---

## Command Line Reference

### Mission Management

```bash
# List available models
./local-ai list-models

# Select a model
./local-ai select-model mistral-7b-q4

# Run a mission
./local-ai run-mission "<goal>"

# Resume a mission
./local-ai resume-mission <mission_id>

# List mission history
./local-ai list-missions

# View mission details
./local-ai get-mission <mission_id>
```

### System Commands

```bash
# Check system status
./local-ai doctor

# Verify offline operation
./local-ai offline-test

# View logs
tail -f ~/.local-ai/logs/agent.log

# View audit trail
tail -f ~/.local-ai/audit.log
```

### Configuration

```bash
# View current configuration
cat ~/.local-ai/config.toml

# Edit configuration
nano ~/.local-ai/config.toml

# Key settings:
# - active_model: which LLM to use
# - autonomy_level: assisted/supervised/autonomous
# - workspace: default workspace directory
```

---

## Configuration Examples

### Example 1: Change Default Model

```toml
# ~/.local-ai/config.toml

[inference]
active_model = "mistral-7b-q3"  # Faster than Q4
backend = "ollama"
```

### Example 2: Enable Autonomous Mode

```toml
[agent]
autonomy_level = "autonomous"  # Run without asking
max_iterations = 50
token_budget = 20000
```

### Example 3: Set Default Workspace

```toml
[paths]
workspace = "/home/user/projects/myapp"
```

---

## Understanding Output

### Mission Result Structure

```
~/.local-ai/missions/<mission_id>/
├── mission.json          # Mission metadata
├── plan.json             # Execution plan
├── observations.json     # Agent observations
├── results.json          # Final results
├── logs/
│   ├── agent.log         # Agent loop logs
│   ├── tools.log         # Tool execution logs
│   └── errors.log        # Any errors
└── artifacts/
    ├── report.md         # Human-readable report
    └── data.json         # Structured data
```

### Example Result

```json
{
  "mission_id": "m-abc123",
  "goal": "Analyze project structure",
  "status": "completed",
  "duration_seconds": 125,
  "findings": {
    "python_files": 24,
    "total_lines": 3421,
    "modules": ["utils", "models", "api"],
    "issues": [
      {
        "severity": "medium",
        "location": "src/utils.py:42",
        "message": "Missing error handling"
      }
    ]
  }
}
```

---

## Autonomy Levels Explained

### Assisted (Default)

```bash
./local-ai run-mission "Delete old logs" --autonomy=assisted

# Agent asks:
#   "About to execute: rm /workspace/logs/*.old"
#   "Approve? (yes/no)"

# You: yes
# Agent executes with your permission
```

**Use when**: Learning, untrusted missions, risky operations

### Supervised

```bash
./local-ai run-mission "Analyze and fix errors" --autonomy=supervised

# Agent runs autonomously
# But you can pause/stop anytime:
#   Press Ctrl+C to pause
#   Type "resume" to continue
#   Type "stop" to abort
```

**Use when**: Trusted missions, want to monitor

### Autonomous

```bash
./local-ai run-mission "Run full analysis" --autonomy=autonomous

# Agent runs to completion
# No interruptions
# All decisions logged
```

**Use when**: Fully trusted, time-sensitive, unattended

---

## Troubleshooting

### Problem: "Model not found"

```bash
./local-ai list-models
# Output: No cached models

# Solution: Download a model
./local-ai select-model mistral-7b-q4
# Wait for download to complete (~5 minutes)

# Verify
./local-ai list-models
# Output: mistral-7b-q4 (5.2 GB)
```

### Problem: "Mission timeout"

```bash
# Mission took > 1 hour

# Solutions:
# 1. Reduce mission scope (simpler goal)
./local-ai run-mission "Analyze just src/main.py"

# 2. Increase timeout (edit config)
# config.toml: time_budget_seconds = 7200

# 3. Check if model is too slow
./local-ai list-models
# Switch to faster model (Q3 instead of Q4)
```

### Problem: "Network error" (offline)

```bash
# Should never happen in offline mode
# If it does:

# 1. Check network policy
cat ~/.local-ai/config.toml | grep network
# Should be: enabled = false

# 2. Check logs
tail ~/.local-ai/logs/agent.log | grep -i network

# 3. Report issue (if it persists)
```

### Problem: "Disk full"

```bash
# ~/.local-ai growing too large

# What to delete:
rm -rf ~/.local-ai/missions/old-*  # Delete old missions
rm -rf ~/.local-ai/logs/*.old       # Delete old logs

# What NOT to delete:
# ~/.local-ai/models/              (keep models)
# ~/.local-ai/database.db          (keep data)
# ~/.local-ai/config.toml          (keep config)

# Check space usage
du -sh ~/.local-ai/*
```

---

## Tips & Tricks

### Tip 1: Save Mission Results

```bash
# Export mission as HTML report
./local-ai get-mission m-abc123 --format=html > report.html

# Export as Markdown
./local-ai get-mission m-abc123 --format=markdown > report.md

# Export as JSON
./local-ai get-mission m-abc123 --format=json > data.json
```

### Tip 2: Batch Processing

```bash
# Run multiple missions sequentially
for goal in "Analyze src/" "Test coverage" "Security scan"; do
  ./local-ai run-mission "$goal"
done

# Each mission saves separately
# View all: ./local-ai list-missions
```

### Tip 3: Custom Workspace

```bash
# Different workspace per mission
export WORKSPACE=/path/to/project1
./local-ai run-mission "Analyze"

export WORKSPACE=/path/to/project2
./local-ai run-mission "Analyze"

# Results isolated per workspace
```

### Tip 4: Monitor in Real-Time

```bash
# Terminal 1: Watch logs
tail -f ~/.local-ai/logs/agent.log

# Terminal 2: Run mission
./local-ai run-mission "Complex analysis"

# See agent thinking in real-time
```

---

## Next Steps

1. **Run first mission** — Try `./local-ai run-mission "List all files"`
2. **Read ARCHITECTURE.md** — Understand how it works
3. **Explore SECURITY.md** — Learn about safety features
4. **Check OFFLINE.md** — Verify offline operation
5. **Review PERFORMANCE.md** — Optimize for your hardware

---

## Support

- **Issues**: [GitHub Issues](https://github.com/local-ai/issues)
- **Discussions**: [GitHub Discussions](https://github.com/local-ai/discussions)
- **Security**: security@local-ai.local (when available)

---

**Happy analyzing!** 🚀

For detailed documentation, see [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md).
