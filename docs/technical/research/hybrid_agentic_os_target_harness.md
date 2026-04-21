<style>
  .glass-card {
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    padding: 24px;
    margin: 16px 0;
    color: #ffffff;
    font-family: 'Outfit', 'Inter', sans-serif;
  }
</style>

<div class="glass-card" markdown="1">

# 🔮 Oracle: Architecture Research for Hybrid Agentic OS Target Harness

## Problem Statement
The OHC Hybrid Architecture needs a formal, published research report detailing its competitive edge against market leaders (AI coding assistant, OpenClaw, Hermes) specifically regarding the Agent Harness execution environment. This provides the blueprint for Implementer agents to build our enterprise-grade bwrap sandbox and proxy bridge.

## Research Report & Core Findings
Our synthesis of the market reveals that robust, production-ready local agents rely on specific isolation and instrumentation primitives:

1. **Deep OS-level isolation**: Utilization of `bwrap --unshare-net` to establish constrained execution environments.
2. **Controlled network egress**: Implementation of `socat` proxy bridging for strict network traffic management.
3. **Sandbox escape prevention**: Pre/post-execution Git repository scrubbing to prevent attacks via filesystem hooks.
4. **Subshell evasion mitigation**: Token-level AST command validation (e.g., `tree-sitter-bash`) prior to execution.
5. **Full-Spectrum Observability**: Deep OpenTelemetry instrumentation across the execution lifecycle.

### Component Interaction (Mermaid)
```mermaid
sequenceDiagram
    participant Agent
    participant Bridge as socat Proxy Bridge
    participant Exec as bwrap Execution Harness
    participant Telemetry as OpenTelemetry

    Agent->>Exec: Dispatch Task
    Exec->>Exec: AST Command Validation (tree-sitter-bash)
    Exec->>Exec: Git Repository Scrubbing
    Exec->>Bridge: Network Egress
    Bridge-->>Exec: Response
    Exec-->>Telemetry: Emit Execution Metrics
    Exec-->>Agent: Task Output
```

## Comparative Matrix: OHC vs Market

| Feature Area | AI Coding Assistant | OpenClaw | Hermes | OHC Target Harness | Gap Assessment |
|--------------|---------------------|----------|--------|--------------------|----------------|
| **Isolation** | `bwrap` OS sandboxes | Docker | Varied | `bwrap --unshare-net` | Must implement strict OS boundaries |
| **Network** | Bridge API | Native | Native | `socat` Proxy Bridging | Critical for controlled egress |
| **Security** | AST parsing | Container bounds | Exec loop | AST + Pre/post Git scrub | Must add Git hook scrubbing |
| **Telemetry** | Custom/Basic | Logs | Traces | Full OpenTelemetry | Integrate OTel across lifecycle |

</div>
