<div markdown="1" style="backdrop-filter: blur(20px) saturate(1.213); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.08); font-family: 'Outfit', 'Inter', sans-serif; color: #ffffff; border-radius: 12px; padding: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Market Audit: Agent Harness Architectures

**Author**: Principal Product Researcher & Oracle (L7)
**Date**: 2026-04-16

## Executive Summary
This report analyzes the Agent Harness architectures of top market competitors—specifically Claude Code and OpenClaw—to identify strategic gaps for One Human Corp (OHC). The goal is to inform OHC's Hybrid Architecture (OHC-HA) on the optimal strategy for sub-agent execution isolation, state management, and observability.

## Competitive Analysis

### Claude Code
Claude Code's leaked source (version 2.1.88) reveals a granular, application-layer harness.
- **Isolation Strategy**: Strict policy mappers and semantic validators (`FsReadRestrictionConfig`, `FsWriteRestrictionConfig`, `NetworkRestrictionConfig`). Uses a dedicated `SandboxManager` utilizing Bubblewrap (bwrap) for deep OS-level namespace sandboxing on Linux.
- **Security Checkers**: Token-Level AST validation validating individual Bash AST nodes (e.g., intercepting unsafe compound commands, redirection operators, Zsh dangerous commands). Uses an `--unshare-net` wrapper with socat bridges.
- **Telemetry**: Violations are trapped by `SandboxManager` and tracked via `SandboxViolationStore`. However, it lacks robust cloud telemetry syncing, storing state ephemerally or in basic JSON caches.

### OpenClaw
OpenClaw takes a different approach, utilizing a flexible, multi-harness registry pattern (`pi-embedded-runner`).
- **Isolation Strategy**: Supports multiple `AgentHarness` plugins that can be dynamically selected based on provider and runtime environment parameters.
- **Harness Policy**: Leverages `resolveAgentHarnessPolicy` for session-scoped strictness requirements, allowing fallback mechanisms (e.g., embedded PI backend).
- **Execution**: The `pi-embedded-runner` manages the actual execution attempts, timeouts, and state compaction, prioritizing robust session orchestration over granular OS-level AST restrictions.

## The OHC "Blue Ocean" Advantage

| Feature Area | Claude Code | OpenClaw | **OHC Vision (Hybrid Harness)** |
| :--- | :--- | :--- | :--- |
| **Isolation Layer** | Config-based mappers | Plugin Harness Registry | **Strict Interceptors + K8s Sidecars** |
| **Security Validation**| High (Bash AST rules) | Low (Delegated) | **High (AST rules + SPIFFE Identity)** |
| **Telemetry Emission** | `SandboxViolationStore` | Runner logs | **Real-time OpenTelemetry to Prometheus** |
| **State Tracking** | Ephemeral / Local DB | Session Runners | **pgvector (AutoDream Consolidation)** |

## Architectural Blueprint
OHC must merge the granular AST-level command validation and config-based mappers of Claude Code with the robust plugin registry of OpenClaw. Additionally, we must embed OpenTelemetry directly into the Harness layer to feed the Central Database (OHC-SIP).

```mermaid
graph TD
    A[OHC Hybrid Agent Harness] -->|AST Validation| B{Policy Engine}
    B -->|Violation| C[OpenTelemetry Emitter]
    B -->|Allowed| D[SPIFFE Auth & Exec]
    C -->|Prometheus Metrics| E[Grafana Dashboards]
    C -->|Structured Logs| F[(pgvector / AutoDream)]
    D --> G[Local SQLite / Cloud Postgres]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,D,G premium;
    class B,C,E,F premium;
```

</div>

## Actionable Roadmap: Feature Gap Missions

Based on the research above, the following actionable missions have been identified for the Implementer agents:

1. **Implement Hybrid Agent Harness with AST Validation and OpenTelemetry**
   - **Status**: [GitHub Issue #5228 Created](https://github.com/onehumancorp/mono/issues/5228)
   - **Goal**: Merge granular AST-level command validation (inspired by Claude Code) with a flexible harness registry (inspired by OpenClaw).
   - **Key Components**:
     - `SandboxManager` with strict read/write configuration.
     - `BashASTValidator` to block unsafe compound commands, redirection operators, and malicious Zsh builtins.
     - OpenTelemetry Hooks emitting `ohc_sandbox_violation_total` to Prometheus.
     - SPIFFE integration for zero-trust authorization before execution.
   - **Implementer Prompt**: Available in the associated GitHub issue.

2. **Implement Async Fork Subagents with Context Inheritance**
   - **Status**: [GitHub Issue #5229 Created](https://github.com/onehumancorp/mono/issues/5229)
   - **Goal**: Allow spawned sub-agents to inherit the parent's full conversational context but execute asynchronously in the background.

3. **Implement Local Memory Directory (MemDir) Fallback for Standalone Mode**
   - **Status**: [GitHub Issue #5230 Created](https://github.com/onehumancorp/mono/issues/5230)
   - **Goal**: A robust, file-based local memory caching fallback.

4. **Implement The Bridge Pattern for Remote Tool Execution**
   - **Status**: [GitHub Issue #5231 Created](https://github.com/onehumancorp/mono/issues/5231)
   - **Goal**: Implement a "Bridge" architecture decoupling local execution environment from remote model orchestration.

5. **Implement Local Shell Task Management with Explicit Eviction**
   - **Status**: [GitHub Issue #5232 Created](https://github.com/onehumancorp/mono/issues/5232)
   - **Goal**: Strict, ID-tracked local task manager to guarantee clean eviction and prevent resource exhaustion.
