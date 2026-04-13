---
status: PENDING
agent: Implementer
---

# Instrumenting Swarm Self-Correction & Deliberation Cycles

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
A core value of OHC is **Absolute Autonomy**, powered by Swarm Self-Correction and deep deliberation via UltraPlans. However, we currently have no quantitative data on how often agents successfully self-correct their tool parameters or how long the multi-agent deliberation cycles take. Without these metrics, we cannot identify failing "Specialized Agent" archetypes or optimize the orchestration overhead of complex architectural changes. We need high-fidelity instrumentation for Tool Auto-Correction and UltraPlan Deliberation.

## Research Report
The current implementation of `ToolParameterAutoCorrection` in `srcs/server/orchestration/service.go` performs heuristic fixes but only emits a standard event log. It lacks a Prometheus counter to track success rates. Similarly, the `UltraPlanManager` in `srcs/server/orchestration/ultraplan.go` manages state transitions for complex plans but does not record duration histograms for the deliberation phases. Competitive analysis indicates that "Deliberation Latency" is a key differentiator for Agentic OS efficiency.

## Design Doc
1. **New Metrics in `srcs/server/telemetry/telemetry.go`**:
   - `ohc_tool_autocorrection_total` (Counter): Labels: `agent_id`, `role`, `status` (success/failure).
   - `ohc_deliberation_phase_duration_seconds` (Histogram): Labels: `plan_id`, `phase`.
2. **Instrumentation Points**:
   - **Auto-Correction**: Inside `Hub.ToolParameterAutoCorrection`, increment the counter based on whether a correction was actually made.
   - **UltraPlan**: Inside `UltraPlanManager.modifyStateMachine`, record the duration since `CreatedAt` or `UpdatedAt` when transitioning between phases (e.g., `PROPOSE` to `CRITIQUE`).
3. **Hybrid Compatibility**: Both metrics must invoke `BufferMetricFunc` to ensure they are captured in Standalone mode.

## Implementation Prompt
Hello Implementer agent! Please implement the following analytics instrumentation:

1.  **Metric Definitions**:
    Update `srcs/server/telemetry/telemetry.go` to define and initialize:
    - `ohc_tool_autocorrection_total`: "Total number of tool parameter auto-corrections attempted by agents."
    - `ohc_deliberation_phase_duration_seconds`: "Duration of UltraPlan deliberation phases."
    Expose helper functions: `RecordToolAutoCorrection(ctx, agentID, role, success bool)` and `RecordDeliberationPhaseDuration(ctx, planID, phase, durationSeconds)`.

2.  **Instrument Tool Auto-Correction**:
    In `srcs/server/orchestration/service.go`, update `ToolParameterAutoCorrection` to call `RecordToolAutoCorrection`.

3.  **Instrument UltraPlan Phases**:
    In `srcs/server/orchestration/ultraplan.go`, calculate the time spent in the current phase before updating to a new phase in `modifyStateMachine` and `UpdatePlanStatus`. Call `RecordDeliberationPhaseDuration`.

4.  **Standalone Buffering**:
    Ensure the new helper functions in `telemetry.go` check `BufferMetricFunc != nil` and write to the local buffer with appropriate JSON payloads and PII redaction.

5.  **Testing**:
    - Add unit tests in `srcs/server/telemetry/telemetry_test.go` for the new metrics.
    - Add integration tests in `srcs/server/orchestration/ultraplan_test.go` verifying that phase transitions record durations.

6.  **Verification**:
    Run `bazelisk test //srcs/server/...` to ensure no regressions.
