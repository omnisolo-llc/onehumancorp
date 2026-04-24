# ActionRisk and Approval Workflow Observability Gap Analysis

## Problem Statement
The KAIROS orchestrator recently implemented the "Draft-for-Review" workflow (as outlined in `[architecture]_ai_agent_department.md`), adding `ActionRisk`, `ApprovalStatus`, and `ProposedContent` to the `Task` and `SharedTask` models. However, the `telemetry` module has not been updated to track these events.

As a result, there is zero visibility into how often AI agents propose high-risk actions, the rate at which human CEOs approve or reject these actions, and the latency introduced by waiting for human approval. Without these metrics, we cannot assess the efficiency of the human-in-the-loop workflow or identify if the approval queue is becoming a bottleneck in either Cloud or Standalone environments.

## Research Report
*   **Current State of Data Models:** `src/server/models/task.go` correctly includes `ActionRisk` and `ApprovalStatus`. `src/server/orchestration/tasks_db.go` correctly persists these fields to PostgreSQL.
*   **Current State of Telemetry:** `src/server/telemetry/telemetry.go` lacks any functions or OpenTelemetry instruments for tracking approvals. A search for `ActionRisk`, `ApprovalStatus`, and `Approval` yields no relevant hits in the telemetry package.
*   **Impact:** The platform operates blind to the performance of the Draft-for-Review process. If a tenant's approval latency spikes, or if an agent hallucinates and begins proposing a massive volume of high-risk actions, the system cannot alert operators or visualize the problem.

## Design Doc
*   **Entity Types & Metrics:**
    *   `AgentApprovalRequestTotal` (Counter): Incremented when a task transitions into a state requiring human approval. Tagged by `tenant_id`, `agent_id`, and `action_risk`.
    *   `AgentApprovalResolutionTotal` (Counter): Incremented when a human operator resolves a pending task. Tagged by `tenant_id`, `agent_id`, and `outcome` (approved, rejected, auto-expired).
    *   `AgentApprovalLatency` (Histogram): Measures the duration between request creation and resolution.
*   **Integration Points:**
    *   `src/server/telemetry/telemetry.go`: Add the aforementioned instruments and corresponding `Record*` functions. Ensure `BufferMetricFunc` is utilized for Standalone mode compatibility.
    *   `src/server/orchestration/tasks.go` (and related DB layers): Instrument the state transitions that handle task creation (if high risk) and task approval updates.
*   **Dashboard Visualization:** A new Grafana dashboard panel should be created to track "Pending Approvals Queue Depth", "Approval Resolution Rate", and "Average Approval Latency".

## Implementation Prompt
Implement the necessary OpenTelemetry metrics in the `telemetry` module to track the AI Agent Approval Workflow.

1.  **Define Telemetry Instruments:** In `src/server/telemetry/telemetry.go`, create a counter for approval requests, a counter for approval resolutions, and a histogram for approval latency.
2.  **Add Recording Functions:** Implement `RecordAgentApprovalRequest`, `RecordAgentApprovalResolution`, and `RecordAgentApprovalLatency`. Ensure they handle PII redaction and `BufferMetricFunc` buffering for Standalone mode, matching the existing patterns in the file.
3.  **Instrument KAIROS:** Update the task state transition logic in the orchestrator to call these new telemetry functions when a task requires approval or is resolved.
4.  **Create Grafana Dashboard:** Provide a JSON configuration for a new Grafana dashboard panel visualizing these metrics.
5.  **Testing:** Add comprehensive unit tests in the telemetry package verifying the new functions, including PII redaction checks via the existing linters.

## Priority
P1

## Estimated Scope
Medium
