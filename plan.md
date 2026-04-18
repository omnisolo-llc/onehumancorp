1.  **Analyze current metrics:** Review `srcs/server/telemetry/telemetry.go` and `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to see how `SyncLatency`, `SyncPayloadSize`, `SyncEscalationsCount`, and `syncDaemonBatchSize` are implemented. Check `srcs/server/orchestration/sync_daemon.go`.
2.  **Add `mode` label to SyncDaemon metrics:** Modify `srcs/server/telemetry/telemetry.go` to add `deployment_mode` to `SyncLatency`, `SyncPayloadSize`, and `syncDaemonBatchSize` counters. Also add a new metric `SyncDaemonErrorTotal` to track errors by `mode`.
3.  **Update `sync_daemon.go`:** Modify `srcs/server/orchestration/sync_daemon.go` to record the metrics with the appropriate mode ("Standalone" or "Cloud"). It's currently only active in Standalone mode (`d.dbWrapper.IsSQLite()`). So it will just log "Standalone". If there are failures in `sendToCloud`, track them in `SyncDaemonErrorTotal`.
4.  **Update Dashboard:** Add three new panels to `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to visualize:
    *   Sync Error Rate by Mode (using `ohc_sync_daemon_errors_total`)
    *   Sync Latency (P95) by Mode (using `ohc_sync_latency_ms`)
    *   Sync Payload Size and Batch Depth (using `ohc_sync_payload_size_bytes` and `ohc_sync_daemon_batch_size`)
    *   Inject OHC Premium Glassmorphism styling using text panels.
5.  **Pre-commit steps:** Follow instructions from `pre_commit_instructions`.
6.  **Submit PR**
