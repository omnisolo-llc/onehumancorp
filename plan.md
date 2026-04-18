<<<<<<< HEAD
1. **Analyze telemetry compliance requirements**:
   - Instruction: "In telemetry or logging code, always apply `RedactInterfacePII` (or an equivalent redaction function) to payload maps before calling `json.Marshal` to prevent PII leakage in multi-tenant environments."
   - Issue 1: In `srcs/server/telemetry/telemetry.go`, `RecordQueueLength` directly constructs a payload map using `fmt.Sprintf` and doesn't apply `RedactInterfacePII`. Wait, currently it uses `fmt.Sprintf` instead of a JSON payload! However, to be consistent with all other `BufferMetricFunc` calls, and to comply with the PII redaction rule for payload maps before calling `json.Marshal`, I need to rewrite `RecordQueueLength` to use a map, redact it, and marshal it.
   - Issue 2: In `srcs/server/orchestration/event_log.go`, `sanitizeHubEvent` takes a `raw` object, calls `json.Marshal(raw)`, and THEN tries to unmarshal, redact, and re-marshal. If unmarshal fails, the unredacted payload is saved! The rule dictates redacting the object BEFORE the first `json.Marshal`. I will update `sanitizeHubEvent` to redact `raw` immediately using `telemetry.RedactInterfacePII(raw)`.

2. **Execute changes**:
   - `srcs/server/telemetry/telemetry.go`: Refactor `RecordQueueLength`.
   - `srcs/server/orchestration/event_log.go`: Refactor `sanitizeHubEvent`.

3. **Verify changes**:
   - Run `bazelisk test //srcs/server/telemetry/...` and `bazelisk test //srcs/server/orchestration/...` to ensure all tests pass.

4. **Complete pre-commit steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

5. **Submit the PR**:
   - Issue ID will be included.
=======
1.  **Extract the Target Tarball**:
    - Ensure `claude-code.tgz` is extracted to `/tmp/claude-code`.
2.  **Locate the Agent Harness**:
    - The target repository implements its Harness in the `SandboxManager` wrapper which configures Bubblewrap (`bwrap`) on Linux, applying seccomp filters and network/filesystem namespaces.
3.  **Analyze the Code**:
    - Search specifically in `/tmp/claude-code/CC-Source/node_modules/@anthropic-ai/sandbox-runtime/dist/sandbox/linux-sandbox-utils.js` for the `wrapCommandWithSandboxLinux` function. This reveals how Bubblewrap (`bwrap`) is configured using `--unshare-net`, `--seccomp`, `--ro-bind`, and `--unshare-pid`.
    - Search for UI/Integration points like `shouldUseSandbox` and `BashTool` wrappers that conditionally apply this harness.
4.  **Synthesize Findings into a Markdown Research Report**:
    - Compile a Markdown Research Report to `docs/research/[security]_agent_harness_audit.md`.
    - The report should be formatted exactly with "Premium" styling (glassmorphism tokens `backdrop-filter: blur(20px)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter'`) inside a `<div markdown="1" style="...">` tag.
    - Include Mermaid charts to depict the architecture gap (e.g., standard Node.js vs. Bwrap isolated harness).
    - It must include the following sections exactly: `Title`, `Problem Statement`, `Research Report`, `Design Doc`, `Implementation Prompt`, `Priority` (P1), and `Estimated Scope` (Large).
5.  **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done**:
    - Run the pre-commit script to verify our changes.
6.  **Create a PR and Submit**:
    - Trigger task completion with a YAML block in the final message to create the `issue`.
>>>>>>> 1a189bd (docs(research): add agent harness audit report)
