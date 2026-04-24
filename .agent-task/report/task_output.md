# Missing Telemetry for Standalone Mode Efficiency Analysis

## Problem Statement
The Standalone mode (desktop/local) uses a SQLite database and buffers telemetry to sync to the Cloud in batches using a `SyncDaemon`. While some metrics are gathered in Grafana via Prometheus, there's a lack of explicit visibility into critical Standalone background operation metrics, making it hard to diagnose queue depth issues and sync bottlenecks. Specifically, we are missing metrics and dashboard panels for the time the `SyncDaemon` spends in the `syncOnce` operation, and we are not surfacing existing metrics like SQLite concurrency throttling (`ohc_sqlite_throttled_request_total`) in our Grafana dashboard.

## Research Report
An analysis of the existing Prometheus metrics setup (`src/server/telemetry/telemetry.go`), the SyncWorker implementation (`src/server/telemetry/sync_daemon.go` and `sync_worker.go`), and the Grafana dashboard configurations (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) reveals the following gaps:

1. **Missing SyncDaemon Duration Metric**: `SyncDaemon` locally buffers and syncs metrics in Standalone mode. The time spent executing the `syncOnce` method is not instrumented. There is an `ohc_sync_latency_seconds` mentioned in the "Sync Daemon Health" panel in Grafana, but it's not actually instrumented or recorded inside `sync_daemon.go`'s `syncOnce` method or `sync_worker.go`.
2. **Missing SQLite Throttling Dashboard Visibility**: The `ohc_sqlite_throttled_request_total` metric is already initialized in `telemetry.go` but it is *missing* from the "Hybrid Telemetry" Grafana dashboard. Tracking this is vital because SQLite concurrency limits throttling write requests will directly impact job queue efficiency in Standalone mode.

## Design Doc
To address these observability gaps, we will:
1. **Instrument `syncOnce` Latency**: Add a `syncDaemonSyncDuration` metric (e.g., `ohc_sync_latency_seconds` as a Histogram) to `telemetry.go` and record it in `SyncDaemon` or `sync_worker.go` to measure the time spent syncing buffered telemetry metrics.
2. **Dashboard Update**: Update the `hybrid-telemetry.json` dashboard to properly reflect `ohc_sync_latency_seconds` in the "Sync Daemon Health" panel and add a new panel to visualize `ohc_sqlite_throttled_request_total` alongside SQLite Lock Contention.
3. **Buffer Redaction/Logging Validation**: Ensure new metrics are properly buffered with PII redaction rules to pass linter validations.

## Implementation Prompt
Update `src/server/telemetry/telemetry.go` to introduce a new Histogram metric `ohc_sync_latency_seconds` (accessible via a new exported function `RecordSyncDaemonLatency(ctx context.Context, duration float64)`).
Instrument `src/server/telemetry/sync_daemon.go` (and/or `sync_worker.go`) to record the execution time of metric synchronization by calling `RecordSyncDaemonLatency`.
Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` (and its copies in `deploy/helm/` or `monitoring/` if needed) to:
- Verify "Sync Daemon Health" panel properly charts `ohc_sync_latency_seconds`.
- Add a new time-series panel for "SQLite Throttled Requests" querying `rate(ohc_sqlite_throttled_request_total[5m])`.

## Priority
P1

## Estimated Scope
Small
