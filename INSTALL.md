# Installation Guide - Local AI Agent

## System Requirements

- **OS**: Windows 11+, macOS 10.13+, Linux (Ubuntu 18.04+)
- **CPU**: 4+ cores
- **RAM**: 8GB minimum (16GB recommended)
- **Storage**: 50GB free (for bundled models)

## Quick Start

### Option 1: Desktop App (Recommended)

```bash
# Extract release bundle
tar xzf local-ai-release.tar.gz
cd local-ai

# Run desktop application
./local-ai-ui
```

### Option 2: Command Line

```bash
# Extract release
tar xzf local-ai-release.tar.gz
cd local-ai

# Check system status
./local-ai doctor

# List available models
./local-ai list-models

# Select a model
./local-ai select-model mistral-7b-q4

# Run a mission
./local-ai run-mission "Analyze project structure"
```

## Offline Operation

All components work **100% offline** after installation:

```bash
# Verify offline capability
./local-ai offline-test
```

No internet required for:
- LLM inference (local models)
- Agent execution
- Memory and recall
- Code analysis
- Document retrieval
- Desktop UI

## Configuration

Configuration file: `~/.local-ai/config.toml`

```toml
[inference]
active_model = "mistral-7b-q4"
backend = "ollama"
ollama_endpoint = "http://localhost:11434"

[agent]
max_iterations = 100
token_budget = 10000
action_budget = 50
time_budget_seconds = 3600
```

## Troubleshooting

### "Doctor check fails"
```bash
./local-ai doctor
```

### "Offline test fails"
Network may be in use. Disable network and retry:
```bash
./local-ai offline-test
```

### "Model not found"
Download a model:
```bash
./local-ai select-model mistral-7b-q4
```

## Building from Source

```bash
./scripts/build-release.sh
```

## Support

- Issues: GitHub Issues
- Documentation: ./ARCHITECTURE.md, ./README.md
