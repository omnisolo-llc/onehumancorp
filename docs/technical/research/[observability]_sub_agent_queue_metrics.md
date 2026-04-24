# Title: Add Mode-Specific Prometheus Metrics to Sub-Agent Queue and Spawner

## Problem Statement
The OHC Hybrid Architecture utilizes `DefaultSubAgentSpawner` in `src/server/orchestration/sub_agent.go` and `QueueManager` in `src/server/orchestration/queue/queue_manager.go` to handle isolated sub-agents. However, existing telemetry methods like `pollCounter.Add(ctx, 1)` and `enqueueCounter.Add(ctx, 1)` lack execution context tags, preventing operators from diagnosing context-specific anomalies. Specifically, the system cannot differentiate between Cloud-native (multi-tenant K8s) and Standalone (local SQLite) queue latency, lock contention delays, or sub-agent spawn error rates.

## Research Report
An audit of `QueueManager` and `DefaultSubAgentSpawner` indicates that while there are counters for basic operations, critical bottleneck indicators are missing mode differentiation. For example, `QueueManager.Acquire()` emits `telemetry.RecordSQLiteLockContention(ctx, "acquire")` which tracks local database lock contention, but there is no equivalent Postgres-specific lock tracking for Cloud deployments. Furthermore, queue dwell time—from job creation until polling—is not instrumented as a duration histogram, and `DefaultSubAgentSpawner.failTask` only modifies database state without emitting mode-labeled Prometheus counter metrics for sub-agent spawn failures.

To enable self-correction of the agent swarm and diagnose disparate performance footprints between environments, mode-specific metrics (`Cloud` vs `Standalone`) must be introduced and visualized on the Hybrid KAIROS dashboards.

## Design Doc
1. **Prometheus Metrics Definition:**
   - Define a new Counter in the `telemetry` package for sub-agent failures (`ohc_sub_agent_spawn_errors_total`) with a `mode` label.
   - Define a new Histogram in the `telemetry` package for queue latency (`ohc_sub_agent_queue_latency_seconds`), tagged with a `mode` label, to measure the time elapsed from `job.CreatedAt` to acquisition.
   - Define a mode-labeled Counter `ohc_sub_agent_lock_contention_total` to replace or supplement `RecordSQLiteLockContention`, generalizing for both PostgreSQL and SQLite.

2. **Code Implementation:**
   - In `src/server/telemetry/telemetry.go`: Implement wrappers `RecordSubAgentQueueLatency(ctx, duration, mode)`, `RecordSubAgentSpawnError(ctx, mode)`, and `RecordSubAgentLockContention(ctx, mode)`.
   - In `src/server/orchestration/queue/queue_manager.go`: Update `Acquire()` to calculate the duration since `job.CreatedAt` and emit `RecordSubAgentQueueLatency`. Record lock contention based on the provider (PostgreSQL vs SQLite) using the `mode` tag.
   - In `src/server/orchestration/sub_agent.go`: Update `DefaultSubAgentSpawner.failTask` and `executeWithRetry` to emit `RecordSubAgentSpawnError(ctx, mode)` upon failure.

3. **Grafana Dashboards:**
   - Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to include three new Text/HTML styled panels for:
     1. Sub-Agent Queue Latency (P95) by Mode.
     2. Sub-Agent Spawn Error Rate by Mode.
     3. Lock Contention Rate by Mode.

## Implementation Prompt
You are an Implementer. Implement the sub-agent telemetry improvements as designed above:
1. Update `src/server/telemetry/telemetry.go` to declare and initialize `SubAgentQueueLatency`, `SubAgentSpawnErrors`, and `SubAgentLockContention` OpenTelemetry metrics, ensuring they accept a `mode` label.
2. Modify `src/server/orchestration/queue/queue_manager.go` to calculate queue dwell time in `Acquire()` and emit `RecordSubAgentQueueLatency`.
3. Update `QueueManager.Acquire` to emit `RecordSubAgentLockContention` appropriately based on whether the provider is Postgres or SQLite.
4. Modify `src/server/orchestration/sub_agent.go` to capture errors inside `DefaultSubAgentSpawner.failTask` and `executeWithRetry` using the new telemetry functions.
5. Update the Grafana dashboard JSON `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to visualize these mode-labeled metrics natively inside Text/HTML panels conforming to OHC styling guidelines.
6. Verify your implementation with `bazel test //...`.

## Priority
P1

## Estimated Scope
Medium
