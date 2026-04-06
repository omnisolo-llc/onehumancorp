---
status: DONE
agent: Implementer
---

# Swarm Queue & SQLite Exhaustion Observability Panels

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
While OHC Hybrid Architecture (OHC-HA) backend exports extensive telemetry, the current Grafana provisioned dashboard (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) is missing critical visualization panels for several metrics. Specifically, metrics tracking task queue bottlenecks (`ohc_task_queue_length`, `ohc_task_processing_latency_seconds`, `ohc_task_failed_total`) and Standalone SQLite retry exhaustion (`ohc_sqlite_retry_exhausted_total`) are unmapped. This masks critical agent task invocation bottlenecks in cloud-native deployments and severe lock failures in standalone setups.

## Research Report
An observability gap analysis comparing backend OpenTelemetry instrumentation against the Grafana provisioning state reveals:
- The backend emits metrics such as `ohc_task_queue_length`, `ohc_task_processing_latency_seconds`, `ohc_task_failed_total`, and `ohc_sqlite_retry_exhausted_total`.
- The Grafana dashboard `hybrid-telemetry.json` lacks visualization panels for these metrics.
- In Cloud-Native mode, monitoring task queue latency and failures is crucial for horizontal pod autoscaling and bottleneck detection.
- In Standalone mode, monitoring SQLite retry exhaustion is critical, as `ohc_sqlite_lock_contention_total` alone does not tell us if the backoff retries successfully handled the contention or completely failed.
- According to the Visual Excellence Mandate, any UI additions (including Grafana custom text panels if configured) should adhere to glassmorphism, but for native Grafana timeseries panels, standard JSON configuration matching the existing theme is sufficient.

## Design Doc
1. **Grafana Dashboards Update**:
   - Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
   - Add a "Task Queue Length" panel utilizing the expression `ohc_task_queue_length` or `ohc_swarm_task_queue_length`.
   - Add a "Task Processing Latency" panel utilizing the expression `rate(ohc_task_processing_latency_seconds_sum[5m]) / rate(ohc_task_processing_latency_seconds_count[5m])` or `ohc_swarm_task_processing_latency_ms`.
   - Add a "Task Failed Rate" panel utilizing the expression `sum(rate(ohc_task_failed_total[5m])) by (error)`.
   - Add a "SQLite Retry Exhaustion" panel utilizing the expression `sum(rate(ohc_sqlite_retry_exhausted_total[5m])) by (operation)`.
2. **Dashboard Verification**:
   - Verify that the panels correctly use the `$datasource` variables.

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. Navigate to `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. Add a new panel for **Task Queue Length** targeting the `ohc_task_queue_length` metric (or `ohc_swarm_task_queue_length`).
3. Add a new panel for **Task Processing Latency** targeting the latency metric.
4. Add a new panel for **Task Failed Rate** targeting `ohc_task_failed_total`. Make sure `legendFormat` is `{{error}}`.
5. Add a new panel for **SQLite Retry Exhaustion** targeting `ohc_sqlite_retry_exhausted_total`. Make sure `legendFormat` is `{{operation}}`.
6. Use `bazelisk test //srcs/server/...` to ensure your JSON syntax hasn't broken any embed tests and that telemetry integration points are still correct.
