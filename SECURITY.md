# Security Model - Local Autonomous AI

This document describes the security architecture, policy engine, permission model, and threat mitigations for the Local Autonomous AI platform.

## Core Principles

1. **Default Deny**: Network disabled by default, filesystem restricted to workspace
2. **Least Privilege**: LLM cannot elevate permissions or modify security rules
3. **Defense in Depth**: Multiple validation layers (schema → policy → sandbox)
4. **Transparent Audit**: All decisions logged, user-inspectable
5. **No Cloud Dependency**: All security decisions local, no external calls

---

## Policy Engine Architecture

```
LLM Output (JSON)
    ↓
[1] Schema Validation ← Check against StructuredAction schema
    ↓
[2] Policy Engine ← Route to policy rules (per tool, folder, risk level)
    ↓
[3] Permission Check ← User autonomy level (Assisted/Supervised/Autonomous)
    ↓
[4] Risk Assessment ← Impact analysis (what could go wrong?)
    ↓
[5] Human Approval ← Ask user if risky (ASK_USER verdict)
    ↓
[6] Sandbox ← Isolate execution (filesystem, network, resources)
    ↓
[7] Tool Execution ← Run with restrictions
    ↓
[8] Result Validation ← Check output before returning to LLM
```

**No action from LLM becomes a system operation without passing all 8 stages.**

---

## Permission Model (5 Levels)

### Level 0: Assisted (Default)
- User is asked before ANY potentially risky action
- Network: **DENY**
- Filesystem: **ASK** (read workspace, **DENY** outside)
- Code execution: **ASK** (any shell command)
- Suitable for: New users, untrusted missions

**Verdicts**: ALLOW, DENY, ASK_USER

### Level 1: Supervised
- Agent executes autonomously, but user can pause/stop
- Network: **DENY**
- Filesystem: **ALLOW** (read/write workspace, **DENY** outside)
- Code execution: **ALLOW** (sandboxed shell)
- Suitable for: Trusted missions, user monitoring

**Verdicts**: ALLOW, DENY (no ASK_USER)

### Level 2: Autonomous
- Agent runs to completion without interruption
- Network: **DENY**
- Filesystem: **ALLOW** (read/write workspace, **DENY** outside)
- Code execution: **ALLOW** (sandboxed shell)
- Suitable for: Fully trusted, time-sensitive missions

**Verdicts**: ALLOW, DENY (no pausing)

### Level 3: Extended (Future)
- Reserved for future multi-agent scenarios
- Allows inter-agent communication
- Parallel execution

### Level 4: Custom (Future)
- User-defined policy rules per tool/folder/risk

---

## Policy Engine Configuration

### Tool-Level Policies

```toml
# ~/.local-ai/security.toml

[tools.file_read]
default_verdict = "ALLOW"
# Can be overridden:
# - Per-folder: [tools.file_read.folders."/workspace/secrets"]
# - Per-pattern: [tools.file_read.patterns."*.key"]

[tools.file_write]
default_verdict = "ASK"  # Ask before writing
protected_paths = ["/workspace/.env", "/workspace/config.yaml"]
# ASK if writing to protected paths

[tools.shell_exec]
default_verdict = "ASK"
dangerous_commands = ["rm -rf", "dd", "format", "delpart"]
# Always ASK before dangerous commands

[tools.network]
default_verdict = "DENY"
# Network always denied unless explicitly enabled per tool
```

### Risk Assessment

Risk levels: CRITICAL, HIGH, MEDIUM, LOW

```
CRITICAL (always ASK or DENY):
- Delete workspace
- Execute shell with 'rm -rf'
- Network access to external hosts
- Write to .env or secrets

HIGH (ASK in Assisted mode):
- Write to workspace root
- Execute any shell command
- Modify code files

MEDIUM (ALLOW if Supervised+):
- Read files in workspace
- List directories

LOW (ALLOW):
- Read-only queries to memory/RAG
- Status checks
```

---

## Sandbox Implementation

### Filesystem Sandbox

**Workspace boundary enforcement**:
- All file operations checked against workspace root
- Path normalization prevents `../../../` escape
- Non-existent file paths allowed (for creation)
- Symbolic links followed safely

**Example validations**:
```rust
validate_read("/workspace/data/file.txt")  → ✅ ALLOW
validate_read("/etc/passwd")                → ❌ DENY
validate_write("/workspace/output.txt")     → Check policy
validate_write("/workspace/../../../etc")   → ❌ DENY (traversal)
```

### Process Sandbox

**Shell execution isolation**:
- Subprocess runs with timeout (default: 5 minutes)
- CPU/memory limits enforced
- Network access denied (by default)
- Can be extended with Docker/Podman in future

**Example**:
```bash
# Shell command from agent
"find /workspace -name '*.py' -type f"

# Executed as:
timeout 300 bash -c 'find /workspace -name "*.py" -type f'
# Runs in subprocess, output captured, process killed on timeout
```

### Memory & Resource Limits

- **Per-mission memory**: < 500MB (budget enforced)
- **Per-tool memory**: < 100MB (subprocess)
- **Timeout**: 5 minutes per tool call
- **Token budget**: Configurable (default 10,000)
- **Action budget**: Configurable (default 50 actions)
- **Time budget**: Configurable (default 1 hour)

---

## Threat Model (10 Categories)

See [THREAT_MODEL.md](THREAT_MODEL.md) for:
- Detailed threat descriptions
- Probability & impact assessment
- Mitigation strategies
- Associated security tests

Quick reference:
1. **Prompt Injection** → Data/instruction separation
2. **Malicious Tool Output** → Result validation
3. **Path Traversal** → Path normalization + workspace boundary
4. **Code Execution** → Sandbox + timeout
5. **Privilege Escalation** → No LLM auto-elevation
6. **Network Exfiltration** → Default-deny network
7. **Infinite Loops** → Loop detection + timeout
8. **Resource Exhaustion** → Budget enforcement
9. **Malicious Documents** → Sandboxed parsing
10. **Supply Chain** → Signed releases + checksums

---

## Autonomy Levels in Practice

### Assisted Mode (Default)

```
User: "Analyze the project and fix all syntax errors"

Agent: "I need to:"
  1. Read all Python files
  2. Parse for errors
  3. Modify files to fix errors
  4. Run tests

Agent → Policy Engine:
  - Tool: file_read, Path: /workspace/*.py
    Verdict: ASK → User approves ✓
  - Tool: file_write, Path: /workspace/src/*.py
    Verdict: ASK → User approves ✓
  - Tool: shell_exec, Cmd: "python -m pytest"
    Verdict: ASK → User approves ✓

Agent executes with approval on each risky action.
```

### Supervised Mode

```
User runs with --autonomy=supervised

Agent executes without asking:
  - Reads files (ALLOW)
  - Writes to workspace (ALLOW)
  - Executes shell (ALLOW)

User can:
  - Pause mission at any time
  - View logs in real-time
  - Stop if concerns arise
  - Resume when ready
```

### Autonomous Mode

```
User explicitly enables with --autonomy=autonomous

Agent executes to completion:
  - No pausing
  - No approval gates
  - Runs unattended

⚠️ Only for trusted missions!
```

---

## Implementing Security Checks

### 1. Schema Validation

```rust
// Reject malformed LLM output
let action: StructuredAction = serde_json::from_str(&llm_response)?;
// Fails if JSON invalid or fields missing
```

### 2. Policy Check

```rust
let verdict = policy_engine.check(
    tool_name: "file_write",
    target: "/workspace/data.txt",
    user_autonomy: Assisted,
);

match verdict {
    ALLOW => execute_tool(),
    DENY => log_and_skip(),
    ASK_USER => show_approval_dialog(),
}
```

### 3. Sandbox Execution

```rust
let result = sandbox.execute_tool(
    tool: file_write,
    args: { path: "/workspace/output.txt", content: "..." },
    timeout: 5min,
    memory_limit: 100mb,
);
```

### 4. Result Validation

```rust
// Ensure tool output is safe to show to LLM
let sanitized = validate_tool_output(result);
// Remove secrets, limit size, check format
```

---

## Audit Logging

All security decisions logged to `~/.local-ai/audit.log`:

```json
{
  "timestamp": "2026-09-05T10:30:00Z",
  "mission_id": "m-12345",
  "event": "policy_check",
  "tool": "file_write",
  "target": "/workspace/data.txt",
  "autonomy_level": "assisted",
  "verdict": "ASK_USER",
  "user_response": "approved",
  "executed": true
}
```

**Audit log is readable and searchable**:
```bash
grep "verdict.*DENY" ~/.local-ai/audit.log
grep "tool.*shell_exec" ~/.local-ai/audit.log
```

---

## Configuration Examples

### Example 1: Strict Workspace Protection

```toml
[policy]
autonomy_level = "assisted"

[tools.file_write]
default_verdict = "ASK"
protected_paths = [
  "/workspace/.env",
  "/workspace/secrets/*",
  "/workspace/.git/*",
]
```

### Example 2: Autonomous Research Mode

```toml
[policy]
autonomy_level = "autonomous"

[tools.file_read]
default_verdict = "ALLOW"

[tools.shell_exec]
default_verdict = "ALLOW"
dangerous_commands = ["rm", "dd", "format"]  # Still denied
```

### Example 3: Network-Enabled (Future)

```toml
[policy]
autonomy_level = "supervised"

[tools.network]
default_verdict = "ALLOW"
whitelist = ["api.example.com", "data.opendata.org"]
```

---

## Security Best Practices

### For Users

1. ✅ **Start with Assisted mode** (default)
2. ✅ **Review mission goals** before running
3. ✅ **Check audit logs** after completing missions
4. ✅ **Use different workspaces** for different projects
5. ✅ **Keep software updated** for security patches
6. ❌ **Don't use Autonomous mode** for untrusted missions
7. ❌ **Don't disable network policy** unless necessary
8. ❌ **Don't run as root/admin** (not necessary, risky)

### For Developers

1. ✅ **Validate all external input** (LLM, documents, network)
2. ✅ **Use schema validation** for LLM output
3. ✅ **Run tools in sandbox** (subprocess, timeout, resource limits)
4. ✅ **Log all security decisions** to audit.log
5. ✅ **Test threat model** scenarios in CI
6. ❌ **Don't trust LLM output** as system instructions
7. ❌ **Don't hardcode credentials** in code
8. ❌ **Don't skip sandbox** for "trusted" tools

---

## Security Testing

All 10 threat categories have test cases in `tests/security/`:

```bash
cargo test --test security_* -- --nocapture
```

Tests verify:
- ✅ Path traversal blocked
- ✅ Network denied by default
- ✅ Shell timeout enforced
- ✅ Loop detection works
- ✅ Budget limits respected
- ✅ Privilege escalation prevented
- ✅ Malicious documents sandboxed
- ✅ Tool output validated
- ✅ Audit logging complete
- ✅ Policy verdicts correct

---

## Reporting Security Issues

If you discover a security vulnerability:

1. **Do NOT open a public GitHub issue**
2. **Email**: security@local-ai.local (when available)
3. **Include**: 
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested mitigation
4. **Timeline**: We aim to respond within 48 hours

---

## Future Enhancements (Phase 15+)

- [ ] Finer-grained policies (per-function, per-line)
- [ ] Policy versioning (rollback old policies)
- [ ] Dynamic policy updates (without restart)
- [ ] External policy validation (optional)
- [ ] Machine-learning-based risk detection
- [ ] Hardware security module (HSM) integration
- [ ] Encrypted audit logs
- [ ] Real-time policy monitoring dashboard

---

**Last Updated**: 2026-09-05  
**Status**: Production-Ready  
**Review Cycle**: Every release (v1.x)  
**Owner**: Security Team
