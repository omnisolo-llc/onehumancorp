<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [harness] Integrate OpenTelemetry for Agent Harness Sandbox Violations

## Problem Statement
The current OHC Agent Harness lacks real-time observability for sandbox violations, making it difficult to debug execution constraints and monitor potential malicious sub-agent activity. Without structured telemetry, we cannot proactively identify edge cases where agents fail due to overly restrictive permissions.

## Research Report & Competitive Analysis
Analysis of the leaked **Claude Code Agent Harness** (v2.1.88) reveals a sophisticated `SandboxViolationStore` that actively tracks, bubbles up, and logs sandbox violations during terminal executions. Their harness utilizes Bubblewrap (`bwrap`) alongside an AST validator to intercept unauthorized commands and path traversals, emitting detailed context back to the user and their backend.

Other frameworks, like **OpenClaw**, rely heavily on basic runner logs, lacking the deep integration into a central telemetry system. **Gstack** focuses on isolated workspaces but does not mandate strict metric emissions for capability denial.

To achieve "Absolute Autonomy" and "Full-Spectrum Observability," OHC must integrate OpenTelemetry directly into the harness to export these violations as structured Prometheus metrics, bridging the gap identified between OHC's current regex-based sandboxing and Claude's deep structural telemetry.

### Comparative Matrix

| Feature | OHC Hybrid Architecture (Current) | Claude Code Harness | Gap / Opportunity |
| :--- | :--- | :--- | :--- |
| **Sandbox Isolation** | Regex-based (`bash_sandbox`) | `bwrap` + AST Validation | **Critical**: Requires deep OS-level sandboxing |
| **Violation Tracking**| Ephemeral logs | `SandboxViolationStore` | **High**: Need durable metric emissions |
| **Telemetry System** | Standard logs | Custom proxy/backend sync | **High**: Must integrate with OpenTelemetry |

## Design Doc
We will integrate the OpenTelemetry Go SDK into the `SandboxManager` and `BashASTValidator` components of the Agent Harness.
Every intercepted invalid command (e.g., path traversal, blocked commands) will generate an OTel Span and emit an `ohc_sandbox_violation_total` metric counter.

### Architectural Blueprint

```mermaid
graph TD
    A[Agent Planner] --> B[Unified Agent Worktree Harness]
    B --> C[BashASTValidator]
    C -->|Invalid Command| D[OpenTelemetry Emitter]
    C -->|Valid Command| E[Sandbox execution]
    D --> F[Prometheus Metrics]
    D --> G[(pgvector / AutoDream)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,E premium;
    class D,F,G premium;
```

## Implementation Prompt
1. Add OpenTelemetry metric initialization to `src/server/harness/sandbox.go`.
2. Increment the `ohc_sandbox_violation_total` counter whenever the AST validator or `bwrap` process blocks an execution attempt.
3. Ensure the metric includes tags for `agent_id`, `violation_type` (e.g., `path_traversal`, `blocked_command`, `network_deny`), and `harness_mode`.
4. Add comprehensive unit tests in `src/server/harness/sandbox_test.go` verifying metric emissions during blocked executions.

## Priority
P1

## Estimated Scope
Medium

</div>
