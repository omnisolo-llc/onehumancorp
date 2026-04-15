1. **Identify the missing `BufferMetricFunc` instrumentation in the specific telemetry files:**
   - I need to check `srcs/server/telemetry/telemetry_bridge.go` to see if `RecordBridgeMessageSent`, `RecordBridgeMessageReceived`, and `RecordBridgeStatus` are using `BufferMetricFunc`.
   - I need to check `srcs/server/telemetry/rag_sync_metrics.go` to see if any `Record*` functions exist that should use `BufferMetricFunc`.
   - I need to check `srcs/server/telemetry/minimax_metrics.go` to see if `RecordMinimaxCall` uses `BufferMetricFunc`.
   - I need to check `srcs/server/telemetry/token_forecast_worker.go` to see if there are missing `BufferMetricFunc` usages in its logic (though it calls `RecordTokenBurnRate`, which might already have it).

2. **Implement `BufferMetricFunc` for `RecordBridgeMessageSent`, `RecordBridgeMessageReceived`, and `RecordBridgeStatus` in `telemetry_bridge.go`:**
   - Modify `telemetry_bridge.go` to include `json` import if needed (and use `telemetry.RedactInterfacePII` since it's in the same package).
   - In `RecordBridgeMessageSent`: Add the standard `if BufferMetricFunc != nil` block. The metric name is "ohc_mesh_bridge_messages_sent_total" or similar, maybe just "mesh_bridge_message_sent".
   - Same for `RecordBridgeMessageReceived` and `RecordBridgeStatus`.

3. **Implement `BufferMetricFunc` for `RecordMinimaxCall` in `minimax_metrics.go`:**
   - Modify `minimax_metrics.go` to add the `BufferMetricFunc` instrumentation to `RecordMinimaxCall`. Pass operation, duration, and error strings in the payload map, redact it, and marshal to JSON.

4. **Add `RecordRAGRecordsSynced` and `RecordRAGSyncError` in `rag_sync_metrics.go` (if they don't exist yet but are implied by the counters) and instrument them with `BufferMetricFunc`:**
   - Provide helper functions to record these metrics. Right now, there are only variables initialized in `initRAGSyncMetrics`.

5. **Ensure PII is scrubbed correctly using `RedactInterfacePII` before passing to `BufferMetricFunc`.**

6. **Create a mission file for the implementer agent in `.agent-task/missions/`.**
   - The user asked me, a Principal Data Scientist (RESEARCHER), to act in my role.
   - The role specifies: "Do NOT implement the feature yourself. Your job is to produce the highest-quality mission briefs possible so that Implementer agents can pick them up and execute flawlessly."
   - Create a mission file `2026-04-15T...Z.md` that strictly adheres to the schema (Title, Problem Statement, Research Report, Design Doc, Implementation Prompt, Priority, Estimated Scope).
   - Ensure the markdown is wrapped in the specific Glassmorphism `<div>`.

7. **Ensure mission details don't violate Groundedness Rule:**
   - I have already explored and verified the existence and content of the `telemetry_bridge.go`, `minimax_metrics.go`, `rag_sync_metrics.go`, and `telemetry.go` files, as well as the behavior of `BufferMetricFunc` and `RedactInterfacePII`.

8. **Execute pre-commit steps and submit.**
