# Title: Add Mode-Specific Prometheus Metrics to Sub-Agent Queue and Spawner

## Problem Statement
The OHC Hybrid Architecture currently utilizes `DefaultSubAgentSpawner` in `src/server/orchestration/sub_agent.go` and `QueueManager` in `src/server/orchestration/queue/queue_manager.go` to handle isolated sub-agents spawned from shared tasks. However, the existing telemetry calls do not distinguish between Cloud-native (multi-tenant K8s) and Standalone (local SQLite) contexts, preventing full-spectrum observability of bottlenecks like queue latency differences, SQLite lock contention vs Postgres lock contention, and sub-agent spawn error rates.

## Research Report
An audit of `QueueManager` reveals that it directly tracks `enqueueCounter` and `pollCounter`, and updates a generic `TaskQueueDepth` gauge without detailed visibility into context-specific throughput or queue dwell times across deployment modes. Similarly, `DefaultSubAgentSpawner` utilizes general logging without fine-grained telemetry to capture `Spawn` timeouts or context-specific retry backoff failures. By adding OpenTelemetry attributes (e.g., `mode` label) to critical path metrics, we can map Cloud vs. Standalone sub-agent performance disparities and expose them through Grafana to enable targeted self-correction.

## Design Doc
1.  **Define Prometheus Metrics in Telemetry Package:**
    *   Add `ohc_sub_agent_queue_latency_seconds` (Histogram) to measure the duration jobs spend in the queue (from `CreatedAt` to `poll` time), tagged with `mode`.
    *   Add `ohc_sub_agent_spawn_errors_total` (Counter) to track failures in `DefaultSubAgentSpawner.executeWithRetry`, tagged with `mode`.
    *   Add `ohc_sub_agent_lock_contention_total` (Counter) to measure database lock delays within `QueueManager.Poll`, tagged with `mode` (`sqlite` vs `postgres`).

2.  **Code Updates:**
    *   **`src/server/telemetry/telemetry.go`:** Declare and initialize the new metrics. Create wrapper functions like `RecordSubAgentQueueLatency(ctx, duration, mode)`, `RecordSubAgentSpawnError(ctx, mode)`, and `RecordSubAgentLockContention(ctx, mode)`.
    *   **`src/server/orchestration/queue/queue_manager.go`:** Update `Poll()` to calculate the duration since `job.CreatedAt` and emit `RecordSubAgentQueueLatency`. If a polling lock retry exceeds typical times, emit `RecordSubAgentLockContention`.
    *   **`src/server/orchestration/sub_agent.go`:** Update `executeWithRetry` and `failTask` to emit `RecordSubAgentSpawnError`.

3.  **Grafana Visualization:**
    *   Update the appropriate Grafana dashboard JSON (e.g., `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json`) to include visualizations for:
        *   Sub-Agent Queue Latency (P95) by Mode.
        *   Sub-Agent Spawn Error Rate by Mode.
    *   Ensure panels follow the "Premium" aesthetic mandate (Glassmorphism, 20px blur, Outfit/Inter typography).

## Implementation Prompt
You are an Implementer. Implement the sub-agent telemetry improvements as designed above:
1.  Update `src/server/telemetry/telemetry.go` to add `SubAgentQueueLatency`, `SubAgentSpawnErrors`, and `SubAgentLockContention` OpenTelemetry metrics, ensuring they accept a `mode` label.
2.  Modify `src/server/orchestration/queue/queue_manager.go` to calculate queue dwell time in `Poll()` and log lock contention issues using the new telemetry functions.
3.  Modify `src/server/orchestration/sub_agent.go` to capture errors inside `DefaultSubAgentSpawner.failTask` and `executeWithRetry` using the new telemetry functions.
4.  Update the Grafana dashboards in `deploy/docker/grafana/provisioning/dashboards/` to visualize these mode-labeled metrics natively inside Text/HTML panels conforming to OHC styling guidelines.
5.  Ensure all unit tests in `sub_agent_test.go` and `queue_manager_loop_test.go` pass and achieve 100% test coverage using `bazel test //...`.

## Priority
P1

## Estimated Scope
Medium
