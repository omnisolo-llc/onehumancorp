# OHC Agent Harness Evolution: Market Research & Gap Analysis

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">

## 🔬 Executive Summary

As the Principal Product Researcher & Oracle (L7), I have analyzed leading AI agent environments—**OpenClaw**, **Hermes Agent**, and **Claude Code**—to identify critical architectural gaps in our OHC (One Human Corp) Agentic OS.

The most pressing gap is the **Agent Harness** layer: the isolated environment where agents execute code, interact with the system, and are observed. OHC currently relies on basic execution without robust isolation, whereas market leaders have established sophisticated Sandbox Run-times.

This report synthesizes these findings and proposes high-impact missions to evolve the OHC Agent Harness to market dominance.

## 📊 Market Analysis: The State of Agent Harnesses

### 1. Claude Code (Anthropic)
**Key Pattern:** Runtime Sandboxing & Granular Interception.
Claude Code employs a sophisticated `@anthropic-ai/sandbox-runtime` which leverages OS-level primitives:
*   **Linux**: Uses `bwrap` (Bubblewrap) for unprivileged sandboxing, creating isolated namespaces, network bridges, and custom mount points.
*   **macOS**: Uses `sandbox-exec` and custom profile monitoring.
*   **Network Control**: Embedded `http-proxy` and `socks-proxy` to intercept, log, and filter all agent outbound traffic.
*   **Violation Store**: Dedicated telemetry for `SandboxViolationStore` to track and metricize out-of-bounds agent actions.
*   **Execution Validation**: Tree-sitter based AST parsing blocks dangerous shell commands (e.g. Zsh blocks, compound bugs).

### 2. OpenClaw
**Key Pattern:** Tiered Isolation Modes & Multi-Tenant Routing.
*   **Tier 1 (Main Session)**: Tools run on host, granting full access for single-user trust.
*   **Tier 2 (Non-Main Session)**: `agents.defaults.sandbox.mode: "non-main"` forces isolated Docker containers per-session.
*   **Access Control**: Strict allow/deny lists for capabilities (`bash`, `read`, `write`, vs `discord`, `gateway`).
*   **Local-First Gateway**: A unified control plane handling tools, sessions, and events routing to isolated workspaces.

### 3. Hermes Agent & Gstack
**Key Pattern:** Execution Telemetry, Ephemeral Context, & Parallel Sprints.
*   Strong focus on ephemeral execution environments that spin up/down instantly.
*   Heavy reliance on structured logs and OpenTelemetry for observability.
*   **Gstack Conductor** isolates workspaces per sprint for parallel agents.

---

## 🆚 OHC Current State vs. Market Reality

| Feature Capability | OHC Current State | Market Standard (Claude/Claw) | Priority Gap |
| :--- | :--- | :--- | :--- |
| **Execution Sandboxing** | Direct execution / basic processes | Docker containers or `bwrap` OS sandboxes | 🚨 Critical |
| **Network Interception** | Native host networking | Local HTTP/SOCKS proxies with whitelist/blacklist | High |
| **Telemetry & Observability**| Basic logging | `SandboxViolationStore` + OpenTelemetry Metrics | High |
| **Capability ACLs** | Unrestricted / Hardcoded | Granular allow/deny lists (e.g., `bash`, `read` only) | Medium |
| **AST-Level Validation** | Basic or none | Tree-sitter based pre-flight validation | Medium |

---

## 🏛️ OHC-Shield Harness Architecture

```mermaid
graph TD
    A[Agent Session] --> B(Harness Proxy / Capability Authorizer)
    B -->|Network Intercept| C{SOCKS/HTTP Proxy}
    C -->|Allowed| D[External Web]
    C -->|Denied| E[(Violation Store DB)]
    B -->|Tool Execution| F{OS-Native Sandbox}
    F -->|AST Check| J[Bash AST Validator]
    J -->|Linux| G[bwrap namespace]
    J -->|macOS| H[sandbox-exec]
    G --> I(Local Tools)
    H --> I(Local Tools)
```

## 🛠️ Proposed Architectural Upgrades (AutoDream Consolidated)

To achieve absolute autonomy while maintaining zero-trust security and full observability, OHC must adopt the **OHC-Shield Harness Architecture**.

1.  **OS-Native Sandboxing (Desktop Mode)**: Implement `bwrap` on Linux to isolate local agent execution without the overhead of Docker.
2.  **Proxy Telemetry Layer**: Inject a transparent local proxy to capture all agent network I/O, emitting OpenTelemetry metrics for every request.
3.  **Sandbox Violation Engine**: Track blocked actions or unauthorized tool usage in PostgreSQL (`tasks`/`events` tables) and visualize via Grafana.
4.  **Granular Capability ACLs**: Implement strict allow/deny capabilities for `bash`, `read`, and `write`.
5.  **Dynamic AST-Level Command Validation**: Utilize a tree-sitter based parser to validate shell commands pre-execution.

</div>
