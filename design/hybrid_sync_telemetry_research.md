Parent: #4909

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [research] Implement Telemetry for Hybrid Sync Escalations

## Problem Statement
The OHC Hybrid Architecture relies on `HybridSyncDaemon` to bridge the local SQLite single-user instance and the Postgres multi-tenant database for Omni-Context synchronization. Specifically, it syncs RAG vectors when `escalation_required` is true. However, the telemetry tracking for these escalations is completely missing in `srcs/server/orchestration/hybrid_sync/hybrid_sync.go`. `RecordSyncEscalation` and `RecordRagEscalation` metrics exist in `telemetry.go` but are never invoked during the `ProcessSync` routine. This gap prevents the swarm from monitoring sync efficiency and error rates in Standalone vs Cloud-Native mode.

## Research Report
An analysis of `srcs/server/orchestration/hybrid_sync/hybrid_sync.go` shows that the `ProcessSync` method successfully fetches and uploads `SyncPayload` objects to the Cloud API from `swarm_memory_embeddings`. However, it fails to log these successful syncing operations to OpenTelemetry using existing global metrics `SyncEscalationsCount` and `RagEscalationCount`. The telemetry definitions `RecordSyncEscalation(ctx context.Context, count int64)` and `RecordRagEscalation(ctx context.Context)` exist in `srcs/server/telemetry/telemetry.go`.

## Design Doc
1. **Integration**: Modify `ProcessSync` in `srcs/server/orchestration/hybrid_sync/hybrid_sync.go`.
2. **Metrics Application**:
   - Call `telemetry.RecordSyncEscalation(ctx, int64(len(payloads)))` after a successful `sendToCloud` execution.
   - Iterate over the payloads and call `telemetry.RecordRagEscalation(ctx)` per payload.
3. **Observability**: Ensure these metrics properly feed into OHC Cloud Grafana dashboards to monitor latency and synchronization status.

## Implementation Prompt
Hello Implementer!
1. Open `srcs/server/orchestration/hybrid_sync/hybrid_sync.go`.
2. Inside `ProcessSync`, right after successfully sending payloads to the cloud (`err := d.sendToCloud(ctx, payloads)` succeeds), record the metrics.
3. Call `telemetry.RecordSyncEscalation(ctx, int64(len(payloads)))`.
4. Call `telemetry.RecordRagEscalation(ctx)` inside a loop over `payloads`.
5. Ensure the file imports `github.com/onehumancorp/mono/srcs/server/telemetry`.
6. Run `bazelisk test //srcs/server/orchestration/hybrid_sync/...` to verify your code.

## Priority
P1

## Estimated Scope
Small

</div>
