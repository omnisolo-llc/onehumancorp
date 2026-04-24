# Add Metrics and Dashboards for AutoDream Task Consolidation and Queue Health

## Problem Statement
The One Human Corp platform operates in both Cloud-native (Kubernetes + PostgreSQL) and Standalone (local SQLite + Bubblewrap) modes. Currently, our telemetry provides general API latencies and token usage, but lacks deep visibility into the asynchronous AutoDream pipeline, particularly task consolidation throughput and queue health. This gap prevents Swarm operators from diagnosing bottlenecks where AI agents might get stuck processing dead-letter jobs, or where SQLite lock contention locally degrades AutoDream performance compared to Postgres in the cloud.

## Research Report
**Hybrid Telemetry Review:**
- We examined `src/server/telemetry/telemetry.go` and `metrics.go` and the `autodream` module usage.
- While `ohc_autodream_consolidation_total` and `ohc_autodream_memories_compressed_total` exist, there is no granular measurement of the *queue depth* or the *processing latency distribution* (e.g., job wait time vs. execution time) across the two deployment modes.
- SQLite shows signs of `sqliteLockContentionCounter` scaling with concurrent Swarm tasks in Standalone, but we lack a correlated metric for how this impacts AutoDream's queue draining rate.

**Observability Gap Analysis:**
- Missing `ohc_autodream_queue_depth` (Gauge) for both PostgreSQL (`SKIP LOCKED` pattern queue) and SQLite.
- Missing `ohc_autodream_job_latency_seconds` (Histogram) to measure the time from job insertion to successful processing.
- Missing Grafana dashboard to visualize AutoDream pipeline health, Swarm dead-letter queue sizes, and mode-specific database contention.

**Bottleneck Hunting:**
- We hypothesize that in Standalone mode, SQLite's single-writer limitation causes sporadic spikes in job latency due to lock contention (`sqliteRetryEventCounter`), whereas Cloud-native PG handles `FOR UPDATE SKIP LOCKED` seamlessly. We need explicit metrics to prove this.

**Cost Efficiency Analysis:**
- We already track `ohc_token_burn_rate_predicted_24h` and `AgentCostEstimateUSD`, but these are not currently mapped per AutoDream consolidation job, making it hard to identify anomalous "stuck" agents burning tokens in background tasks.

## Design Doc
**Architecture:**
- **Metric Definitions:**
  - Add `AutoDreamQueueDepth` (`ohc_autodream_queue_depth`) as a gauge measuring the current number of pending consolidation tasks.
  - Add `AutoDreamJobLatency` (`ohc_autodream_job_latency_seconds`) as a histogram measuring the end-to-end processing time of a background task.
- **Data Source Integration:**
  - Update `src/server/telemetry/telemetry.go` and `metrics.go` to declare and initialize these metrics using the standard OpenTelemetry API.
  - Inject recording logic into `src/server/workers/autodreamWorker` (or equivalent worker process) where dequeuing and completion occur.
  - Implement a periodic poller in the telemetry daemon (or worker loop) to emit the queue depth gauge.

**Dashboard Integration:**
- Create a new Grafana JSON dashboard defining panels for "AutoDream Queue Depth", "Job Processing Latency (Cloud vs Standalone)", and "Database Lock Contention vs Queue Size".

## Implementation Prompt
1. In `src/server/telemetry/telemetry.go`, register a new gauge `AutoDreamQueueDepth` (`ohc_autodream_queue_depth`) and a new histogram `AutoDreamJobLatency` (`ohc_autodream_job_latency_seconds`).
2. In the module that handles the AI Job Queue (e.g., `src/server/pipeline` or AutoDream worker), add logic to:
   - Record the time taken to process an AI job and log it to `AutoDreamJobLatency`.
   - Periodically query the database for the count of pending AI jobs (handling both PostgreSQL and SQLite appropriately) and record it using `AutoDreamQueueDepth`.
3. Ensure the context passed to the metrics includes attributes indicating the current deployment mode (Cloud vs Standalone).
4. Provide a sample Grafana dashboard JSON file in `src/server/telemetry/dashboards/` (or update an existing one) to visualize these new metrics.
5. Create E2E tests validating that the metrics endpoint exposes the new AutoDream metrics after simulating job creation.

## Priority
P1

## Estimated Scope
Medium
