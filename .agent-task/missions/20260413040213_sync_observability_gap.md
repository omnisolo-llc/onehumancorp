# Title: Fix AutoDream Sync Engine Observability Gap (Metrics & Grafana)

## Problem Statement
The `AutoDreamSyncEngine` in Standalone Mode (`srcs/server/sync/autodream_sync.go`) is partially instrumented. It directly increments `telemetry.SyncFailedCount` and `telemetry.SyncCompletedCount`, but these metrics are NOT visualized in the Grafana dashboard (`hybrid-telemetry.json`). Conversely, the Grafana dashboard has a "Sync Daemon Health" panel that attempts to display `ohc_sync_latency_seconds` and `ohc_sync_daemon_batch_size`, but these metrics are never actually recorded in `autodream_sync.go`.

## Research Report
- Auditing `srcs/server/sync/autodream_sync.go` reveals that `telemetry.RecordSyncLatency`, `telemetry.RecordSyncPayloadSize`, and `telemetry.RecordSyncDaemonBatchSize` are completely unused.
- The Grafana dashboard at `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` lacks visualization for the active counters `ohc_sync_completed_count` and `ohc_sync_failed_count`.
- This mismatch prevents the Data Scientist and the human CEO from accurately observing the health, latency, and success rates of local-to-cloud synchronization.

## Design Doc
- **Telemetry Hooks**: Update `srcs/server/sync/autodream_sync.go` to wrap the sync operations with `time.Now()` tracking, then call `telemetry.RecordSyncLatency(ctx, duration.Seconds())`, `telemetry.RecordSyncPayloadSize(ctx, int64(len(jsonData)))`, and `telemetry.RecordSyncDaemonBatchSize(ctx, int64(len(payloads)))`.
- **Grafana Configuration**: Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include "Sync Completed" and "Sync Failed" metrics in the existing "Sync Daemon Health" panel (or a new panel).

## Implementation Prompt
Hello Implementer agent! Please execute the following tasks:
1. Open `srcs/server/sync/autodream_sync.go`. Inside `syncEmbeddingCache` and `syncAgentMissions`, measure the execution time using `start := time.Now()` and call `telemetry.RecordSyncLatency(ctx, time.Since(start).Seconds())` upon completion.
2. Inside `sendToCloud` in `autodream_sync.go`, measure the size of the serialized `jsonData` and call `telemetry.RecordSyncPayloadSize(ctx, int64(len(jsonData)))`.
3. Inside `syncEmbeddingCache` and `syncAgentMissions`, record the batch size using `telemetry.RecordSyncDaemonBatchSize(ctx, int64(len(payloads)))`.
4. Open `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`. Find the panel with `"title": "Sync Daemon Health"`. Add new Prometheus targets to display `sum(rate(sync_completed_count[5m]))` and `sum(rate(sync_failed_count[5m]))`.
5. Verify your changes by running tests: `~/go/bin/bazelisk test //srcs/server/sync/...` and `~/go/bin/bazelisk test //deploy/...`.

## Priority
P1

## Estimated Scope
Small
