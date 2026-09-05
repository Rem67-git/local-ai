# Threat Model & Security Tests

## Threats & Mitigations

### 1. Prompt Injection (CRITICAL)
**Threat**: LLM receives malicious instructions in documents/tool output
**Mitigation**: Strict input validation, untrusted data separation, instruction templates
**Test**: `test_prompt_injection_blocked`

### 2. Path Traversal (CRITICAL)
**Threat**: Agent escapes workspace via ../ or symlinks
**Mitigation**: PathValidator in sandbox, canonicalize checks
**Test**: `test_path_traversal_blocked`

### 3. Arbitrary Code Execution (CRITICAL)
**Threat**: Unrestricted shell_exec or code execution
**Mitigation**: Sandbox, allowlist, budget limits, permission engine
**Test**: `test_shell_exec_restricted`

### 4. Privilege Escalation (HIGH)
**Threat**: Agent gains system access beyond sandbox
**Mitigation**: Process isolation, capability dropping, AppContainer on Windows
**Test**: `test_no_privilege_escalation`

### 5. Network Exfiltration (HIGH)
**Threat**: Agent sends data to external servers
**Mitigation**: Default DENY network, explicit opt-in only
**Test**: `test_network_blocked_by_default`

### 6. Infinite Loops (HIGH)
**Threat**: Agent loops forever, exhausting resources
**Mitigation**: Loop detection, budget enforcement, timeout
**Test**: `test_loop_detection_triggers`

### 7. Resource Exhaustion (MEDIUM)
**Threat**: Memory/CPU DoS
**Mitigation**: Budget limits (tokens, actions, time, memory)
**Test**: `test_budget_enforced`

### 8. Malicious Documents (MEDIUM)
**Threat**: Crafted PDFs/code trigger vulnerabilities
**Mitigation**: Sandboxed parsing, size limits, format validation
**Test**: `test_malicious_document_safe`

### 9. Tool Output Injection (MEDIUM)
**Threat**: Tool output manipulated to inject commands
**Mitigation**: Output validation, escaping, sandboxing
**Test**: `test_tool_output_validated`

### 10. Supply Chain Attack (LOW)
**Threat**: Compromised dependencies
**Mitigation**: Dependency pinning, vendoring for offline, checksums
**Note**: Offline-first reduces exposure
