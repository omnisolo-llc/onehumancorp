# OHC Hybrid Mode Telemetry Gap Analysis & Insights

## 1. Executive Summary

As OneHumanCorp’s operational complexity increases with scaling Swarm agents in a hybrid setting (both Cloud-native Kubernetes implementations and local Standalone environments), consistent, fine-grained telemetry ensures swift issue resolution and long-term optimization. Based on recent code and metrics investigations into the `src/server/telemetry` module, `src/server/orchestration/tasks.go` and Grafana dashboards, I have identified multiple gaps in logging, visualization, and metric ingestion that must be addressed to properly observe differences in throughput, queueing, and task execution behavior between hybrid and standalone modes.

## 2. Key Findings

### 2.1 Missing Mode Segmentation on Claim Contention
*   **Gap:** Although `TaskClaimContentionTotal` is recorded and exported via `telemetry.RecordTaskClaimContention(ctx, "redis")`, it is missing corresponding metrics for Postgres/SQLite lock failures. More importantly, this metric isn't properly segmented by the deployment mode (e.g., `cloud` vs `standalone`). This makes identifying whether contention is due to Redis distributed locks in Cloud or database locks in Standalone impossible.
*   **Recommendation:** Instrument mode detection or tags on `RecordTaskClaimContention` dynamically based on the current context (`tm.db.IsSQLite()` check). Add a visualization for this counter (by `mode`) into `kairos_hybrid_metrics.json` to directly compare cloud and standalone locking throughputs.

### 2.2 Unvisualized SubAgent Queue Delays
*   **Gap:** `SubAgentQueueDelayHistogram` is exported via `telemetry.RecordSubAgentQueueDelay` and emitted when a task transitions from `PENDING` to `IN_PROGRESS` in `tasks.go`. However, this critical histogram—which represents the wait time of our queue—is entirely missing from the core Grafana dashboards (`kairos_hybrid_metrics.json` and `hybrid-telemetry.json`). There is only a reference to queue lengths, not delays.
*   **Recommendation:** Add a P95 and P99 latency visualization panel for `SubAgentQueueDelayHistogram` in `kairos_hybrid_metrics.json`, faceted by deployment mode.

### 2.3 `Harness*Latency` Dashboard Drift
*   **Gap:** Metrics like `HarnessInitLatency`, `HarnessDbIoLatency`, and `HarnessExecutionLatency` exist in `telemetry.go` and are captured. The `harness_efficiency.json` dashboard aims to graph these, but relies on a mismatch in the queries. They attempt to filter by `deployment_mode` which is correctly exposed by the metric, but they are isolated to a single dashboard rather than being correlated with agent task completion delays.
*   **Recommendation:** Unify these metrics to provide a single pane of glass in the primary `kairos_hybrid_metrics.json` where Harness Latency and Queue Delays can be tracked side-by-side to find execution bottlenecks.

### 2.4 Lack of Context in Agent Execution Traces
*   **Gap:** `RecordAgentExecutionTrace` only tags by `agent_id` and `trace_type`. While this provides volume metrics, it lacks task-level dimensions (like `task_priority` or `mission_id`) or environment context. Without this, tracing a high execution volume back to a specific stuck or looping task in SQLite vs. Postgres is difficult.
*   **Recommendation:** Extend `RecordAgentExecutionTrace` to optionally take `mission_id` or `task_id`.

## 3. Top Bottlenecks

1.  **Row-Level vs. Distributed Contention:**
    The discrepancy between SQLite file-level locks (`tm.mu.Lock()`) and Redis Redlock (`Nx().Ex()`) means task claiming bottlenecks behave entirely differently. In Standalone, throughput is gated by disk I/O and mutex contention. In Cloud, it's gated by Redis network roundtrips. We cannot currently compare these.
2.  **Queue Wait vs. Execution Wait:**
    Tasks sit in `PENDING` state and `SubAgentQueueDelayHistogram` measures this wait. However, without dashboard visualization of queue wait vs execution duration (`HarnessExecutionLatency`), operators cannot tell if a slowdown is due to "too few workers" or "slow workers".
3.  **Missing DB Contention Metrics:**
    `ohc_sqlite_lock_contention_total` and `ohc_postgres_lock_contention_total` are referenced in dashboards but are disconnected from the actual task claim logic. There are no explicit telemetry calls tracking SQLite or Postgres query delays within `ClaimTask`.

## 4. Next Steps & Proposed Issues

To close the observability gap and ensure efficient Swarm scaling, we should implement the following tasks:

### Issue Brief: Enhance Hybrid Observability and Queue Delay Visualization

**Problem Statement:**
For a non-technical business owner, the "Manager" agent is expected to be constantly available. When the agent is delayed in processing tasks (e.g., waiting in a queue or struggling to claim a task due to system load), the owner experiences this as the app "hanging" or an AI agent being unresponsive.

Currently, the Swarm operators lack visibility into these delays because:
1. `SubAgentQueueDelayHistogram` is recorded in code but not visualized in the primary hybrid metrics dashboards.
2. Task claim contention (when multiple workers try to grab the same task) is only recorded for Redis, ignoring Standalone (SQLite) and Cloud DB (Postgres) failures, and lacks mode segmentation.
3. It is impossible to correlate if a delayed response is due to a slow queue or a slow agent execution environment (Harness latency).

This prevents operators from effectively debugging slow AI response times and routing bottlenecks in Cloud vs. Standalone modes.

**Design Doc:**
1.  **Dashboard Enhancement**: Inject panels into `kairos_hybrid_metrics.json` to visualize the P50, P90, and P99 of `SubAgentQueueDelayHistogram`. These should be faceted by the deployment mode.
2.  **Dashboard Unification**: Bring critical `harness_efficiency.json` metrics (Init Latency, Execution Latency) into the primary hybrid dashboard for side-by-side queue vs execution correlation.
3.  **Contention Instrumentation**: Update `ClaimTask` in `src/server/orchestration/tasks.go` to emit `TaskClaimContentionTotal` not just for Redis misses, but also for database-level contention (e.g., query timeouts, busy DB errors). Ensure the metric includes a tag specifying whether the contention occurred in Cloud (Redis/Postgres) or Standalone (SQLite) mode.

**Implementation Prompt:**
Update the telemetry infrastructure and Grafana dashboards to provide end-to-end visibility of task delays.
- Modify the `ClaimTask` flow to ensure task claim failures (whether from Redis or DB locks) are recorded in `TaskClaimContentionTotal` with the correct deployment mode label.
- Update `kairos_hybrid_metrics.json` to include visualizations for `SubAgentQueueDelayHistogram` (P95/P99 latency).
- Correlate these delays by adding `HarnessExecutionLatency` visualizations to the same primary dashboard.
- The user-facing outcome is a single pane of glass where operators can instantly see if slow AI responses are caused by queue backups or slow execution environments, across both Standalone and Cloud deployments.

**Priority:** P1
**Estimated Scope:** Medium

