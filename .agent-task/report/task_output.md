# [observability] Hybrid Telemetry and Bottleneck Analysis

## Title
Observability 🔍 (Hybrid Telemetry & Bottleneck Resolution)

## Problem Statement
The OHC platform operates in a dual mode: Cloud-native (PostgreSQL + K8s) and Standalone (SQLite + local execution). Current telemetry analysis reveals significant discrepancies in execution efficiency and error rates between these two modes. Specifically, in Standalone mode, there is a pronounced bottleneck surrounding SQLite database access under high concurrency (Swarm operations). This manifests as elevated `ohc_sqlite_lock_contention_total` metrics, which directly correlate with increased task queue latency (`ohc_task_queue_length`) and higher task failure rates (`ohc_task_failed_total`). Furthermore, there is a lack of adequate visualization for these specific failure domains in the existing Grafana dashboards, leaving the human CEO blind to the degraded performance experienced by local deployments.

## Research Report
- **Goal**: Analyze the execution data and identify the root causes of the throughput and error rate differences between Cloud and Standalone modes. Propose a comprehensive solution encompassing both observability and structural improvements to mitigate the identified bottlenecks.
- **Data & Findings**:
  - **Metric Analysis**: A review of `src/server/telemetry/telemetry.go` confirms the existence of metrics specifically tracking lock contention (`postgresLockContentionCounter`, `sqliteLockContentionCounter`).
  - **Standalone Bottleneck**: In Standalone mode, functions querying database records often hit concurrency limits due to SQLite's lock behavior (`SQLITE_BUSY`). The telemetry metric `ohc_sqlite_lock_contention_total` acts as a leading indicator of swarm degradation.
  - **Impact on Queues**: The lock contention directly impacts the AI Job Queue, leading to an inflation of `ohc_task_queue_length` as tasks are delayed or retried.
  - **Error Rates**: Persistent lock contention results in exhausted retries (`sqliteRetryExhaustedCounter`), culminating in an increase in `ohc_task_failed_total`.
  - **Observability Gap**: While the metrics exist in the backend (e.g., `sqliteLockContentionCounter.Add()`), there is a gap in translating these raw metrics into actionable insights via the Grafana dashboards (`monitoring/dashboards/`). A dedicated view for Standalone Swarm Health is missing.
- **Competitive Context**: Unlike cloud-only platforms (Shopify), OHC's promise of a robust local offline mode requires parity in reliability. The current SQLite locking issues violate this promise.

## Design Doc
1. **Application-Level Fallback Locking**:
   - To mitigate SQLite lock contention without relying solely on SQLite's internal retries, implement an application-level fallback locking mechanism in Standalone mode for high-concurrency database queries (e.g., `ClaimTask`, `PollTasks`).
   - This involves using a `sync.Mutex` alongside the database call. The pattern should use a non-blocking `TryLock` to detect immediate contention, followed by a blocking `Lock` if necessary, ensuring a disciplined approach to database access.
2. **Dashboard Enhancement**:
   - Create a new Grafana dashboard specifically targeting "Standalone Swarm Health".
   - This dashboard must visualize the correlation between `ohc_sqlite_lock_contention_total`, `ohc_task_queue_length`, and `ohc_task_failed_total`.
   - Incorporate visualizations for `sqliteRetryExhaustedCounter` and `sqliteThrottledRequestCounter`.
3. **Telemetry Refinement**:
   - Ensure that high-frequency metric buffering for SQLite contention is functioning correctly to avoid synchronous serialization overhead, which could further exacerbate performance issues.

## Implementation Prompt
"Implement an application-level fallback lock using `sync.Mutex` for critical database queries in the Standalone SQLite provider (specifically targeting functions like `ClaimTask` and `PollTasks` that exhibit high contention). The mechanism should attempt `TryLock` first, and if unsuccessful, block on `Lock()`. Ensure that `telemetry.RecordSQLiteLockContention` is called appropriately when contention is detected to maintain accurate metrics. Additionally, create a new Grafana dashboard configuration file in `monitoring/dashboards/` named `standalone_swarm_health.json` that visualizes the relationship between SQLite lock contention, task queue length, and task failure rates."

## Priority
P0

## Estimated Scope
Medium
