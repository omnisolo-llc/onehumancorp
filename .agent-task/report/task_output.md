# Issue Brief: Implement Telemetry Pipeline Contention Optimization

## Problem Statement
The current hybrid mode telemetry architecture generates significant network contention when syncing telemetry buffer events. In Standalone Mode, local SQLite writes perform well, but when transitioning to Cloud-Native operations, the `mcp_sync_worker` aggressively pushes bulk telemetry metrics and OpenTelemetry traces to the central `ohc-sip` cloud database. This process saturates local I/O and creates substantial lock contention in the centralized Redis and Postgres instances (especially around the `ohc_autodream_memories` synchronization). The user needs this solved without altering the platform's core multi-tenant / standalone hybrid capability.

## Research Report
- **Review of Architecture**: Telemetry events map back to the Swarm's "Shared Task List" operations. Wait times for agent state transitions (via `ohc_kairos_transition_duration_seconds`) spike when the underlying DB transactions in `statemachine.go` and `queue_manager.go` are blocked by telemetry table syncs.
- **Log Review**: High counts of `context deadline exceeded` in `sync_daemon.go` during peak Swarm operation.
- **Data Gap**: A detailed breakdown of Cloud vs. Standalone performance reveals that local processing uses simple transactions, whereas the central sync pipeline attempts full table delta synchronization simultaneously across pods.

## Design Doc
### Proposed Solution: Hybrid Telemetry Throttling & Batching
1. **Adaptive Batching Window**: Introduce an adaptive backoff and chunk size modifier in `srcs/server/telemetry/sync_worker.go`. During high lock contention (detected via `ohc_kairos_transitions_total` error ratios), the worker should shrink payload size and increase `sleep` duration.
2. **Postgres LWW Handling**: When pushing to PostgreSQL, instead of an unconditional `INSERT/UPDATE`, the sync worker must use CRDT Last-Writer-Wins logic with a conditional update (`WHERE excluded.updated_at > [table].updated_at`) to ensure late arriving standalone telemetry does not overwrite more recent Cloud mode agent states.
3. **Redis Pub/Sub Coordination**: Use the `Teammate Mesh` to broadcast `TELEMETRY_SYNC_START` and `TELEMETRY_SYNC_STOP` events. This allows agents to pause non-critical background data processing while heavy syncs occur.

## Implementation Prompt
- Refactor `srcs/server/telemetry/sync_worker.go` to implement an adaptive batch size algorithm based on previous payload duration and error rates.
- Update `srcs/server/telemetry/telemetry_bridge.go` to use the `WHERE excluded.updated_at` LWW conditional check when writing to the remote store.
- Add OpenTelemetry metrics `ohc_telemetry_sync_backoff_duration_seconds` (Histogram) and `ohc_telemetry_batch_size` (Gauge) to track the efficacy of the new throttling mechanism.
- Ensure the Grafana dashboard (`hybrid-telemetry.json`) is updated to include these new metrics.

## Priority
P1

## Estimated Scope
Medium

```yaml
issue_title: "[telemetry] Hybrid Telemetry Synchronization Contention Optimization"
issue_priority: "P1"
issue_description: "Implement adaptive batching and LWW conflict resolution in the telemetry sync daemon to prevent database lock contention during standalone-to-cloud handoffs."
issue_todo_list:
  - [ ] Implement adaptive backoff in sync_worker.go
  - [ ] Implement LWW conflict resolution in telemetry_bridge.go
  - [ ] Add ohc_telemetry_sync_backoff_duration_seconds and ohc_telemetry_batch_size metrics
  - [ ] Update hybrid-telemetry.json Grafana dashboard
issue_label: ["observability", "performance", "high-impact"]
```
