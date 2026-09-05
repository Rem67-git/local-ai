# Release Guide - Local Autonomous AI

This document describes the release process, versioning strategy, distribution channels, and final checklist for the Local Autonomous AI platform.

## Versioning Strategy

**Semantic Versioning**: `MAJOR.MINOR.PATCH`

- **MAJOR** (1.0.0) — Production release, all 14 phases complete, security audit passed
- **MINOR** (1.1.0) — Feature additions (new tools, agents, capabilities)
- **PATCH** (1.0.1) — Bug fixes, security patches, performance tuning

**Version Format**:
- Development: `1.0.0-dev.YYYYMMDD`
- Release Candidate: `1.0.0-rc.1`, `1.0.0-rc.2`, etc.
- Release: `1.0.0`

## Pre-Release Checklist

### Code Quality
- [ ] All phases (0-14) marked COMPLETE in PHASES.md
- [ ] Zero compiler warnings: `cargo build --release 2>&1 | grep warning`
- [ ] All tests passing: `cargo test --all` (unit, integration, security, agent, offline)
- [ ] Code coverage > 80% (reported by CI)
- [ ] No hardcoded secrets in code (grep for API keys, tokens)
- [ ] Documentation updated (ARCHITECTURE.md, SECURITY.md, DECISIONS.md)

### Security
- [ ] All 10 threat model tests pass: `cargo test --test security_*`
- [ ] Security audit completed (internal or external)
- [ ] THREAT_MODEL.md mitigations verified
- [ ] Sandbox isolation tested (path traversal, privilege escalation)
- [ ] Offline operation verified: `local-ai offline-test`
- [ ] No telemetry enabled by default

### Build Artifacts
- [ ] Release binary builds: `./scripts/build-release.sh`
- [ ] Binary size < 50MB (uncompressed), < 20MB (compressed)
- [ ] Tauri desktop app builds on Windows 11
- [ ] All bundled dependencies included (LLM model, SQLite schema)
- [ ] Installation guide reviewed (INSTALL.md)

### Distribution
- [ ] GitHub release page created with:
  - [ ] Release notes (CHANGELOG.md excerpt)
  - [ ] Checksums (SHA256) of all artifacts
  - [ ] System requirements clearly stated
  - [ ] Installation instructions linked
- [ ] Artifacts uploaded:
  - [ ] `local-ai-<version>-windows.exe` (installer)
  - [ ] `local-ai-<version>-macos.dmg` (coming)
  - [ ] `local-ai-<version>-linux.AppImage` (coming)
  - [ ] `local-ai-<version>-source.tar.gz` (source archive)
- [ ] Digital signatures created and verified
- [ ] Release documented on docs site

### User Documentation
- [ ] README.md final version
- [ ] INSTALL.md tested (fresh install walkthrough)
- [ ] ARCHITECTURE.md current and accurate
- [ ] SECURITY.md comprehensive
- [ ] THREAT_MODEL.md complete with mitigations
- [ ] FAQ in README.md covers common questions
- [ ] Video walkthrough recorded (optional, recommended)

### Final Testing
- [ ] Installation from scratch (clean Windows 11 VM)
- [ ] First mission runs successfully
- [ ] Offline operation verified (network disabled)
- [ ] Performance baseline measured (mission completion time)
- [ ] No memory leaks (monitored for 1+ hour)
- [ ] Graceful shutdown (no dangling processes)

## Release Process

### Step 1: Create Release Branch

```bash
git checkout -b release/v1.0.0
```

### Step 2: Update Version Number

```bash
# core/Cargo.toml
[package]
name = "local-ai"
version = "1.0.0"

# apps/desktop/package.json
{
  "version": "1.0.0"
}

# Update in source code:
# core/cli/src/main.rs: const VERSION: &str = "1.0.0";
```

### Step 3: Update CHANGELOG

Add to CHANGELOG.md:
```markdown
## [1.0.0] - 2026-09-05

### Added
- Local-first autonomous agent platform (all 14 phases complete)
- LLM runtime abstraction (Ollama integration, llama.cpp support)
- Agent execution loop with mission planning and decomposition
- Policy engine with configurable permissions (filesystem, shell, code, documents)
- Filesystem sandbox with path traversal prevention
- Memory system with context history, semantic recall, and pruning
- RAG (Retrieval-Augmented Generation) for document indexing
- Desktop UI (Tauri + React) with real-time mission logs
- Multi-agent supervisor pattern with specialist delegation
- Security threat model (10 categories, all mitigated)
- Offline packaging with bundled LLM and zero-dependency distribution
- Complete installation guide and offline verification

### Security
- Default-deny network policy
- Path traversal prevention with sandbox validation
- Loop detection and stalled-action handling
- Budget enforcement (tokens, actions, time)
- All 10 threat model mitigations implemented and tested

### Performance
- ~100ms LLM inference latency (Mistral 7B Q4)
- <10MB memory overhead per mission
- Efficient SQLite storage (mission history, memory)
- Optimized document retrieval (inverted indexing)

### Breaking Changes
None (first release)

### Known Limitations
- Single-agent workflow (multi-agent in phase 10+)
- Limited language support (Python, Rust, JavaScript, Go)
- Model selection hardcoded (customization in future releases)
```

### Step 4: Build Release Artifacts

```bash
./scripts/build-release.sh
```

Expected output:
```
🔨 Building Local AI release...
...
📦 Packaging release...
✅ Release built: dist/
   - Binary: dist/local-ai
   - Desktop app: dist/bundle/
🔍 Verifying offline capability...
✅ Offline test passed
✅ Offline packaging complete!
```

### Step 5: Generate Checksums

```bash
cd dist
sha256sum local-ai* > checksums.txt
cat checksums.txt
```

### Step 6: Create GitHub Release

```bash
gh release create v1.0.0 \
  --title "Local Autonomous AI v1.0.0" \
  --notes "$(cat CHANGELOG.md | head -50)" \
  dist/local-ai-*
```

### Step 7: Publish Release

1. Go to https://github.com/local-ai/releases
2. Review release page
3. Publish release (mark as "Latest")

### Step 8: Post-Release

- [ ] Announce on social channels (if applicable)
- [ ] Update docs site
- [ ] Close milestone in GitHub
- [ ] Thank contributors

## Distribution Channels

### Primary (Recommended)
1. **GitHub Releases** — Direct download, checksums, release notes
2. **Homebrew** (macOS) — `brew install local-ai` (optional, future)
3. **Windows Package Manager** (Windows) — `winget install local-ai` (optional, future)

### Secondary (Future)
- Docker images (local-ai:latest, local-ai:v1.0.0)
- Snapcraft (Linux)
- Conda (Python community)

## Support & Feedback

After release, monitor for:
- GitHub issues (bug reports)
- Installation failures (different OS versions)
- Performance feedback (mission duration, resource usage)
- Feature requests

Establish support response SLA:
- Critical bugs: fix within 24 hours
- Installation issues: respond within 48 hours
- Feature requests: review within 1 week

## Hotfix Process

If critical bug found post-release:

1. Create hotfix branch: `git checkout -b hotfix/v1.0.1`
2. Fix bug in code
3. Bump PATCH version (1.0.0 → 1.0.1)
4. Run all tests
5. Update CHANGELOG.md
6. Tag and release: `git tag v1.0.1`
7. Push release artifacts

## Long-Term Maintenance

After v1.0.0, plan releases:
- **v1.1.0** (Q4 2026): Multi-agent supervisor, additional tools, performance tuning
- **v1.2.0** (Q1 2027): Advanced RAG, knowledge graphs, user plugins
- **v2.0.0** (Q2 2027): Distributed agents, cloud-optional sync, mobile companion

---

**Release Manager**: Core team  
**Release Cadence**: Quarterly (v1.x), Annual (v2.x)  
**Support Window**: Current + 1 previous minor version
