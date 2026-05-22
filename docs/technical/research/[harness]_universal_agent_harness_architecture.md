<div markdown="1" style="backdrop-filter: blur(20px) saturate(1.213); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.08); font-family: 'Outfit', 'Inter', sans-serif; color: #ffffff; border-radius: 12px; padding: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Market Audit: Universal Agent Harness Architecture

**Author**: Principal Product Researcher & Oracle (L7)

## Problem Statement
One Human Corp (OHC) aims to build the world's most autonomous and aesthetically superior Agentic Operating System. Currently, AI agents require robust isolation, rapid browser interaction, and granular tool execution environments to operate effectively. Our competitors have pioneered various components of this: Claude Code offers granular OS-level AST restrictions, OpenClaw provides a flexible harness plugin registry, and gstack achieves sub-second latency via persistent browser daemons. However, no single solution consolidates these features with the premium aesthetics and telemetry mandates required by OHC.

The gap lies in creating a unified "Universal Agent Harness" that combines deep security validation, high-performance web orchestration, and dynamic runtime adaptation, fully integrated into the OHC Hybrid Architecture (OHC-HA) and backed by full-spectrum OpenTelemetry.

## Research Report

### Competitive Analysis

1. **Claude Code (v2.1.88)**
   - **Architecture**: A rigorous, application-layer harness focusing on zero-trust execution.
   - **Strengths**: Implements strict `SandboxManager` policies with granular OS-level and Bash AST node validation. It intercepts unsafe compound commands, redirection operators, and specific shell builtins (e.g., Zsh dangerous commands). It effectively isolates filesystems and network layers via `bwrap`.
   - **Weaknesses**: Lacks sub-second latency for UI orchestration, as it's primarily terminal-bound. Stores telemetry ephemerally or in local JSON caches without real-time cloud aggregation.

2. **OpenClaw**
   - **Architecture**: A multi-harness registry pattern (`pi-embedded-runner`).
   - **Strengths**: Extensible. Provides multiple `AgentHarness` plugins dynamically selected based on provider and runtime environment parameters. Fallbacks are handled gracefully.
   - **Weaknesses**: Delegates heavy lifting to underlying execution engines; validation is less granular than Claude's AST parsers.

3. **gstack**
   - **Architecture**: An HTTP CLI layer wrapping a long-lived Chromium daemon.
   - **Strengths**: Specifically engineered for sub-second latency browser interactions. Persistent state ensures cookies, tabs, and login sessions survive across tool calls.
   - **Weaknesses**: Focused narrowly on the browser; lacks the deep OS-level isolation provided by tools like `bwrap`.

### OHC vs Market

| Feature Area | Market Standard | **OHC Vision (Universal Hybrid Harness)** |
| :--- | :--- | :--- |
| **Isolation Strategy** | Either config mappers (Claude) or Plugin Registries (OpenClaw) | **Hybrid**: Strict AST Interceptors + Extensible K8s Sidecar Registry |
| **Security Validation** | Mixed (High in Claude, Low in others) | **Maximum**: Bash AST validation backed by SPIFFE Identity and `bwrap`/`sandbox-exec` |
| **Browser Execution** | Ephemeral / High Latency | **Sub-second Persistent Daemon** (gstack model) with glassmorphism preview |
| **Telemetry & State** | Local JSON or session logs | **Real-time OpenTelemetry to Prometheus** & state in pgvector (AutoDream) |

### Design Doc

The Universal Agent Harness will be a tri-layered system:

1.  **The Registry Layer (The Orchestrator)**
    *   Inspired by OpenClaw, this layer dynamically loads execution environments (`AgentHarness` plugins) based on the sub-agent's needs (e.g., Node.js, Python, or purely Web).
    *   It interfaces with the OHC-SIP (Central Database) to manage session state via `pgvector`.
2.  **The Security Layer (The Sentinel)**
    *   Inspired by Claude Code, every command dispatched through the harness must pass through a `BashASTValidator`.
    *   Utilizes `bwrap` (Linux) or `sandbox-exec` (macOS) to strictly enforce filesystem and network access.
    *   SPIFFE identities are injected securely into the environment for secret-less auth.
3.  **The Browser Daemon Layer (The Navigator)**
    *   Inspired by gstack, a long-lived headless Playwright/Chromium daemon runs alongside the harness.
    *   Agents communicate with the daemon via local HTTP POST commands, ensuring sub-second latency for UI-based tasks.

```mermaid
graph TD
    A[OHC Universal Harness Registry] -->|Dispatch| B{Policy & AST Validator}
    B -->|Allow| C[SPIFFE Auth Layer]
    B -->|Deny| D[OpenTelemetry Violation Hook]
    C -->|Browser Task| E[Persistent Browser Daemon]
    C -->|OS Task| F[bwrap / sandbox-exec]
    E --> G[Sub-second UI Execution]
    F --> H[Isolated OS Execution]
    E -.-> I[OHC-SIP State pgvector]
    F -.-> I
    D --> J[Prometheus Metrics]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,C,E,F,G,H,I premium;
    class B,D,J premium;
```

## Implementation Prompt

**Role**: Principal Backend Engineer
**Task**: Implement the `Universal Agent Harness` based on the research doc.

1.  **Create the Harness Registry**: Under `src/server/harness/`, implement a plugin registry capable of loading execution strategies dynamically (similar to OpenClaw's `pi-embedded-runner`).
2.  **Integrate AST Validation**: Port the logic for Bash AST validation. Implement `BashASTValidator` in `src/server/harness/security.go` to parse and reject unsafe compound commands and redirection operators before they hit `bwrap`.
3.  **Integrate Persistent Browser Daemon**: Create a persistent Playwright/Chromium daemon manager in `src/server/harness/browser.go`. It must maintain long-lived sessions and expose a local HTTP interface for sub-agents to achieve sub-second execution latency.
4.  **Telemetry Integration**: Add OpenTelemetry hooks. Every blocked execution MUST emit `ohc_harness_violation_total` and every execution must measure `ohc_harness_execution_duration_ms`. Ensure PII is redacted using `RedactInterfacePII` before any JSON serialization.
5.  **State Management**: Ensure all command histories and session checkpoints are synced to the OHC-SIP using `pgvector`.
6.  **Testing**: Write comprehensive unit tests for `BashASTValidator` covering at least 10 different shell attack vectors. Ensure 100% test coverage.

**Acceptance Criteria**:
- `bazelisk test //src/server/harness/...` passes with 100% coverage.
- Telemetry metrics are correctly registered in Prometheus.
- The persistent browser daemon starts correctly and maintains state across multiple isolated calls.

## Priority & Scope
- **Priority**: P0
- **Estimated Scope**: Large

</div>