<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [observability] Hybrid Telemetry Gap Analysis & Standalone Observability Enhancements

**Title**: Implement Dashboard Visualization for Standalone Mode Database Metrics (SQLite Throttling and Retries)

**Problem Statement**:
While the OHC platform instruments key performance indicators for both Cloud (PostgreSQL) and Standalone (SQLite) modes, there is a critical observability gap in our Grafana dashboards. The codebase correctly records metrics such as `ohc_sqlite_throttled_request_total` and `ohc_sqlite_retry_event_total` (in `srcs/server/telemetry/telemetry.go`), which are vital for understanding database lock contention and performance in Standalone mode. However, these metrics are completely absent from the Grafana dashboards (`hybrid-telemetry.json` and `ohc-hybrid.json`). As a result, swarm operators cannot visually detect when Standalone deployments suffer from database throttling or excessive retries, hindering Swarm health assessments and bottleneck hunting in local contexts.

**Research Report**:
1. **Hybrid Telemetry Review**: An analysis of `srcs/server/telemetry/telemetry.go` revealed that OpenTelemetry instrumentation exists for tracking Swarm agent efficiency, database query performance, and network partitions. Key metrics like `ohc_sqlite_lock_contention_total` and `ohc_sqlite_retry_exhausted_total` are recorded.
2. **Observability Gap Analysis**: A search of the Grafana dashboard configurations (`deploy/docker/grafana/provisioning/dashboards/`) showed that while some SQLite metrics are visualized (e.g., `ohc_sqlite_lock_contention_total` and `ohc_sqlite_retry_exhausted_total`), critical throughput and contention indicators are missing. Specifically, there are no panels for `ohc_sqlite_throttled_request_total` (indicating when write operations are throttled by the concurrency limiter) and `ohc_sqlite_retry_event_total` (indicating when transactions are retried due to lock errors).
3. **Bottleneck Insights**: The omission of these metrics masks early warning signs of database performance degradation in Standalone mode. While lock contention is visualized, the upstream impact (throttling and intermediate retries) remains hidden. This obscures the difference in throughput efficiency between Cloud-native K8s execution and local SQLite environments.
4. **Swarm Health Assessment**: The Swarm's ability to self-correct and execute efficiently in Standalone mode relies heavily on the `TaskQueue` and `SubAgentWorker` interacting with the local SQLite database. Without visibility into throttling and retries, operators cannot determine if agents are getting stuck or delayed due to local resource contention versus actual mission complexity.

**Design Doc**:
- **Entity Types**: Grafana Dashboard Panels, Prometheus Metrics (`ohc_sqlite_throttled_request_total`, `ohc_sqlite_retry_event_total`).
- **Key Relationships**: The new panels should be added to the existing "Hybrid Database Telemetry" or "Standalone Mode" sections of the `hybrid-telemetry.json` and `ohc-hybrid.json` dashboards. They should correlate visually with the existing lock contention and retry exhaustion panels.
- **Integration Points**: No new code instrumentation is needed. The integration solely involves updating the Grafana JSON models to query the existing Prometheus metrics exported by the OHC backend.

**Implementation Prompt**:
Update the Grafana dashboard definitions (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and `deploy/docker/grafana/provisioning/dashboards/ohc-hybrid.json`) to include new visualization panels for `ohc_sqlite_throttled_request_total` and `ohc_sqlite_retry_event_total`. These panels should be grouped with the existing SQLite observability metrics to provide a comprehensive view of Standalone mode database health. The user-facing outcome will be that operators monitoring a Standalone deployment will immediately see if database operations are being throttled or retried excessively, enabling proactive troubleshooting of Swarm task delays. The acceptance criteria dictate that the dashboards successfully load and display time-series data for these metrics when queried.

**Priority**: P1
**Estimated Scope**: Small

</div>
