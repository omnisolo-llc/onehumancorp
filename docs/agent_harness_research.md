<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03) !important; font-family: 'Outfit', 'Inter', sans-serif !important; border-radius: 12px; padding: 24px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔬 OHC Agent Harness: Market Research & Architecture Evolution

**Prepared by:** Principal Product Researcher & Oracle (L7)
**Date:** April 2026

## 1. Executive Summary

To achieve absolute autonomy and secure swarm execution, OHC must evolve its existing command runner into an advanced, secure **Agent Harness**. Based on an in-depth technical audit of leading frameworks—Claude Code, OpenClaw, Hermes Agent, and Gstack—this report defines the architectural gaps and the implementation roadmap for OHC's next-generation Agent Harness.

## 2. Competitive Architectural Analysis

### 2.1 Claude Code (Anthropic)
*   **Isolation & Sandboxing:** Utilizes a robust `SandboxManager` with strict filesystem and network restrictions. It supports a "dangerouslyDisableSandbox" fallback, prompting users when operations are blocked.
*   **Terminal Safety:** Employs advanced Bash AST parsing to detect and block malicious or obfuscated commands (e.g., blocking `zmodload`, obfuscated flags, or `jq` system functions).
*   **Harness Abstraction:** Wraps execution environments seamlessly, injecting context through custom CLI hooks and managing state persistently in internal harness paths (`~/.claude/`).

### 2.2 OpenClaw
*   **Local-First Control Plane:** Operates a local Gateway daemon that manages agent sessions, tools, and multi-channel events seamlessly.
*   **Container Sandboxing:** Uses per-session Docker sandboxing for non-main execution, strictly allowing safe tools (bash, process, read, write) while denying risky ones (browser, canvas, cron) by default.

### 2.3 Hermes Agent (Nous Research)
*   **Self-Improving Loop:** Integrates a procedural memory loop, building skills from experience and tracking state using an FTS5 session search.
*   **Serverless Persistence:** Leverages backends like Daytona and Modal to hibernate agent environments when idle, providing cost-effective scalability.

### 2.4 Gstack (Garry Tan)
*   **Specialized Roles & Pipelines:** Implements a strict pipeline (Think → Plan → Build → Review → Test → Ship → Reflect) using specialized virtual agents.
*   **Safety & Execution:** Features power tools like `/careful` (warns before destructive commands) and `/freeze` (locks edits to a specific directory), ensuring safe multi-agent execution.

## 3. OHC Feature Gap & Target Architecture

OHC currently lacks the strict containerized isolation, granular filesystem/network permissioning, and safety guardrails (like command AST validation and edit freezing) observed in the market.

### Target Architecture

```mermaid
graph TD
    subgraph OHC_Agent_Harness [OHC Agent Harness]
        style OHC_Agent_Harness fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.1),stroke-width:1px
        A[Input Command / Task] --> B{Sandbox Manager}
        B -->|Sandbox Enabled| C[AST Validation & Security Checks]
        B -->|Dangerously Disabled| D[Direct Execution with Warnings]
        C -->|Passed| E[Containerized Shell / Worktree]
        C -->|Failed| F[Reject & Notify Oracle]
        E --> G[Full-Spectrum Observability / Telemetry]
        G --> H[Output / State Persistence]
    end
```

### OHC vs Market Comparison

| Feature | OHC Current | Market Best-in-Class (Claude/OpenClaw/Gstack) | OHC Target |
| :--- | :--- | :--- | :--- |
| **Command Safety** | Basic Execution | AST parsing, Dangerous command blocklist, `/careful` guardrails | **Advanced AST Parsing & Destructive Warnings** |
| **Isolation** | Shared Process | Per-session Docker, Worktrees, Network/FS Allow-lists | **Sandboxed Worktrees & Strict FS/Network Policies** |
| **State Management**| Basic Logging | Persistent FTS5 Memory, Isolated Harness Paths | **pgvector integration with Isolated Internal Paths** |
| **Telemetry** | Minimal | High-fidelity OpenTelemetry, Supabase Edge Functions | **Prometheus/Grafana via OpenTelemetry** |

## 4. Implementation Roadmap

1.  **Core Sandbox Interface (`srcs/server/agents/sandbox/`)**: Implement the `SandboxManager` to handle filesystem read/write restrictions, network host filtering, and the `dangerouslyDisableSandbox` logic.
2.  **Terminal Security (`srcs/server/agents/terminal/`)**: Build an AST-based command parser to preemptively block obfuscated shell commands and alert on destructive actions (e.g., `rm -rf`).
3.  **Observability Hooks (`srcs/server/telemetry/`)**: Instrument the harness with OpenTelemetry to track execution durations, block events, and sandbox violations.

</div>
