# Local Autonomous AI Platform

A **local-first, privacy-first, offline-first** platform for running autonomous AI agents on your machine. Execute complex multi-step missions (analysis, planning, coding, testing, reporting) entirely offline—no cloud API calls, no data sent elsewhere.

## 🎯 Mission

Build a secure, extensible runtime for autonomous agents that:
- Runs completely offline after installation
- Executes missions with human oversight
- Manages its own memory, tools, and permissions
- Cannot access or modify files outside the workspace
- Cannot make unintended network connections
- Recovers from errors and resumes interrupted missions

**This is NOT a chatbot.** It's an agent runtime.

## 🚀 Quick Start

### Prerequisites
- Windows 11 Pro (currently), or Linux/Mac (coming)
- 8GB+ RAM
- 4+ CPU cores
- SSD (recommended)
- ~5GB free disk space (for bundled LLM)

### Installation

1. **Download** the latest release from [Releases](https://github.com/local-ai/releases) (coming soon).

2. **Run the installer**:
   ```bash
   local-ai-setup.exe
   ```
   This extracts the executable, downloads the bundled LLM model, and sets up the database.

3. **Verify setup**:
   ```bash
   local-ai doctor
   ```
   If all checks pass, you're ready.

### First Mission

1. **Open the UI**:
   ```bash
   local-ai
   ```
   A desktop window opens.

2. **Define a mission**:
   - **Goal**: "Analyze the Python files in the project and list all syntax errors"
   - **Workspace**: `/path/to/project`
   - **Autonomy**: "Assisted" (ask me before dangerous actions)

3. **Run**:
   Click "Start Mission". The agent will:
   - Read files from your workspace
   - Analyze code
   - Report findings in real-time logs
   - Save results to `~/.local-ai/missions/<mission_id>/`

## 🏗️ Architecture

```
┌─────────────────┐
│  Desktop UI     │ (Tauri + React)
│  (Logs, Stats)  │
└────────┬────────┘
         │
    ┌────▼────────────────────────┐
    │  AGENT RUNTIME              │
    │  (Planner, Memory, Policy)  │
    └────┬───────────────┬────────┘
         │               │
    ┌────▼────┐    ┌────▼─────┐
    │ LLM      │    │ Tools    │
    │ (Local)  │    │ (FS, etc)│
    └─────────┘    └────┬─────┘
                        │
                    ┌───▼──────────┐
                    │ SANDBOX      │
                    │ (Restricted) │
                    └──────────────┘
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

## 📋 Core Concepts

### Missions
A **mission** is a goal statement:
- *"Analyze the test coverage in src/ and identify gaps"*
- *"Fix all compilation errors in the project"*
- *"Generate a summary of the API schema"*

The agent breaks it into sub-goals, executes them, and reports findings.

### Autonomy Levels
1. **Assisted** — agent asks before any risky action (default)
2. **Supervised** — agent acts autonomously but you can pause/stop
3. **Autonomous** — agent runs to completion without interruption (must be explicitly enabled)

### Tools
Built-in tools the agent can use:
- **Filesystem**: read/write/search files (confined to workspace)
- **Shell**: execute commands (sandboxed)
- **Code**: parse, modify, and analyze code
- **Documents**: ingest and search PDFs, markdown, etc.
- **Memory**: recall past missions and learned patterns

### Policy Engine
Enforces rules like:
- ✅ Read files in workspace
- ✅ Write to workspace
- ❌ Access files outside workspace
- ❌ Make unintended network requests
- ❓ Ask user before deleting files

Policies are configurable per tool, folder, and risk level.

## 🔒 Security & Privacy

**No cloud. No telemetry. No data leaves your machine.**

- ✅ All LLM inference is local (llama.cpp/Ollama)
- ✅ All files stay on disk (no cloud storage)
- ✅ All logs are local (JSON files, searchable, auditable)
- ✅ Network is denied by default
- ✅ Filesystem sandbox prevents escape attempts
- ✅ Policy engine validates every action
- ✅ Code is open-source (audit it yourself)

See [SECURITY.md](SECURITY.md) for threat model and mitigations.

## 💾 Offline Operation

After installation, the agent runs **completely offline**:
- LLM model is bundled (no model download)
- Database is local (SQLite)
- No external API calls
- No internet required

Verify with:
```bash
local-ai offline-test
```

## 🔧 Development

### Building from Source

**Prerequisites**:
- Rust 1.70+
- Node 18+
- Python 3.10+ (for embeddings model)

**Clone and build**:
```bash
git clone https://github.com/local-ai/local-ai.git
cd local-ai
cargo build --release
npm --prefix apps/desktop install
npm --prefix apps/desktop run tauri build
```

### Project Structure
```
local-ai/
├── CLAUDE.md               # Development guide
├── ARCHITECTURE.md         # Architecture details
├── DECISIONS.md            # Technology choices
├── PHASES.md               # Development roadmap
├── SECURITY.md             # Security model
├── THREAT_MODEL.md         # Threats and mitigations
├── apps/desktop/           # Tauri + React UI
├── core/                   # Agent runtime (Rust)
├── inference/              # LLM integration
├── tools/                  # Built-in tools
├── sandbox/                # Isolation
├── database/               # Schema
└── tests/                  # Test suites
```

See [PHASES.md](PHASES.md) for development roadmap (15 phases).

### Running Tests

```bash
# Unit tests
cargo test --lib

# Integration tests (requires local LLM)
cargo test --test '*'

# Security tests
cargo test --test security_*

# Offline operation test
local-ai offline-test
```

## 📚 Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** — System design, component overview, data flow
- **[DECISIONS.md](DECISIONS.md)** — Technology choices (Tauri, Rust, SQLite, etc.)
- **[SECURITY.md](SECURITY.md)** — Policy engine, sandbox, threat model
- **[THREAT_MODEL.md](THREAT_MODEL.md)** — Identified threats and mitigations
- **[OFFLINE.md](OFFLINE.md)** — Offline guarantees and verification
- **[PHASES.md](PHASES.md)** — Development roadmap (15 phases)

## 🤝 Contributing

Contributions welcome! Please:
1. Read [CLAUDE.md](CLAUDE.md) for development workflow
2. Follow the security checklist in [SECURITY.md](SECURITY.md)
3. Add tests for new features
4. Update docs if changing architecture

## 📄 License

[MIT](LICENSE.txt) — See [LICENSES.txt](LICENSES.txt) for bundled dependencies.

## ❓ FAQ

**Q: Will you add cloud sync?**  
A: No. Privacy-first means no mandatory cloud. Optional export is possible.

**Q: Can I run this on my laptop?**  
A: Yes! It's optimized for 8GB+ RAM and runs fine on older hardware with quantized models.

**Q: How long does a mission take?**  
A: Depends on complexity and model. Simple missions (file analysis) take 30 sec–2 min. Complex missions (multi-file coding) can take 5–15 min.

**Q: Can I customize the tools?**  
A: Yes, tools are pluggable. See [ARCHITECTURE.md](ARCHITECTURE.md) for the tool interface.

**Q: What models are supported?**  
A: Any GGUF-quantized model (Llama 2, Mistral, Qwen, etc.). Default is Mistral 7B (Q4).

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/local-ai/issues) (coming soon)
- **Discussions**: [GitHub Discussions](https://github.com/local-ai/discussions) (coming soon)
- **Email**: contact@local-ai.local

---

**Built with ❤️ for privacy, autonomy, and offline-first computing.**
