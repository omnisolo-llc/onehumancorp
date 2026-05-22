<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔬 OHC Market Research Report: Agent Harness Architecture Audit

## Problem Statement
Current OHC Agent Harness lacks the robust isolation, state management, and semantic verification seen in state-of-the-art systems like OpenClaw, Hermes Agent, and Claude Code. We need to implement a modern Harness architecture to safely and efficiently execute autonomous AI agents.

## Research Report

### Market Analysis
After deep technical audits of the leading agent frameworks:
- **Claude Code**: Focuses heavily on user interaction via TS/React CLI (`ink`). Implements strict security through complex Bash AST parsing (`bashSecurity.ts`, `sedValidation.ts`, `pathValidation.ts`). It treats the harness as a sophisticated interpreter rather than a strict OS-level container.
- **Gstack**: Uses `bwrap` (Bubblewrap) for rigorous sandbox execution (`Dockerfile.sandbox`), isolating agents completely from the host system while maintaining low overhead.
- **OpenClaw & Hermes**: Rely on Podman and `fly.io` for execution and deployment, emphasizing cloud-native ephemeral environments.

### Gap Analysis for OHC
1. **Isolation:** OHC needs Bubblewrap (`bwrap`) for local sandboxing, matching Gstack's efficiency while running locally.
2. **Security:** OHC needs semantic validation of bash commands via AST parsing (like Claude Code) before passing them to the sandbox.
3. **Telemetry:** OHC needs OpenTelemetry instrumentation around tool executions.
4. **State:** OHC needs a centralized Context Manager synchronizing local (SQLite) and cloud (PostgreSQL) states via PowerSync.

### Comparative Table: OHC vs Market

| Feature | Claude Code | Gstack | OpenClaw | OHC (Proposed) |
| :--- | :--- | :--- | :--- | :--- |
| **Sandbox Technology** | Node.js Process | Bubblewrap (`bwrap`) | Podman | Bubblewrap (`bwrap`) |
| **Command Validation** | Advanced AST Parser | Basic Regex | None | Advanced AST Parser |
| **State Sync** | Local Only | Cloud Only | Ephemeral | Local/Cloud (PowerSync) |
| **Observability** | Console Logs | Datadog | Prometheus | OpenTelemetry/Prometheus |

## Design Doc

### Architecture

```mermaid
graph TD
    A[Agent Planner] --> B(Validation Layer)
    B -->|AST Parsing & Security Check| C{bwrap Sandbox Layer}
    C --> D[Bash Execution]
    C --> E[Python Scripts]
    C --> F(OpenTelemetry Wrap)
    F --> G[Prometheus/Grafana]
    C --> H[PowerSync State Tracker]
    H --> I[(Local SQLite)]
    H --> J[(Cloud PostgreSQL)]

    classDef glass fill:rgba(255,255,255,0.05),stroke:rgba(255,255,255,0.2),stroke-width:1px,color:#fff,backdrop-filter:blur(20px);
    class A,B,C,D,E,F,G,H,I,J glass;
```

1. **Sandbox Layer:** `bwrap` wrapper for executing bash commands and Python scripts.
2. **Validation Layer:** AST parser to validate `bash` commands before execution to prevent destructive actions outside the sandbox.
3. **Telemetry Layer:** OpenTelemetry wrapper around every tool execution.
4. **State Sync Layer:** PowerSync integration for local-to-cloud synchronization of agent context.

### API Contracts
```typescript
interface SandboxExecutionRequest {
  command: string;
  env: Record<string, string>;
  workdir: string;
}

interface SandboxExecutionResponse {
  stdout: string;
  stderr: string;
  exitCode: number;
}
```

## Implementation Prompt
Implement a Bubblewrap (`bwrap`) based Agent Harness Sandbox in Go.

1.  **File Path:** `src/server/agents/harness/bwrap_sandbox.rs`
2.  **Functionality:** Create a struct `BwrapSandbox` that takes a `SandboxExecutionRequest` and executes the command using `bwrap`.
3.  **Security:** Mount the `workdir` with read-write access. Mount `/usr` and `/lib` as read-only. Unshare network (`--unshare-net`) unless explicitly requested.
4.  **Telemetry:** Wrap the execution in an OpenTelemetry span, recording the command, exit code, and execution time.
5.  **Testing:** Create `src/server/agents/harness/bwrap_sandbox_test.rs` with 100% coverage, ensuring successful execution and proper isolation (e.g., verifying network access is blocked).
6.  **E2E Test:** Create an E2E test to verify the complete Sandbox flow. Start from the home page after user login via the UI. Navigate the full feature flow by clicking links and buttons. After the process completes, assert the final product matches what the design docs describe. Every path must be covered with 100% E2E test coverage. Do not mock network requests.

## Priority
P0

## Estimated Scope
Large

</div>
