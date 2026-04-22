<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [Observability] Hybrid Telemetry Gap Analysis

## Title
Resolve Hybrid Observability Gap: Cloud vs Standalone Telemetry Discrepancies 📊

## Problem Statement
The OHC platform currently suffers from fragmented observability across its Cloud-native and Standalone Desktop deployments. Business operations lack consistent visibility because critical throughput and error metrics are not uniform between the environments. Furthermore, certain AI agent sub-systems and per-tenant cost meterings are entirely blind in the Standalone mode, making it difficult to evaluate swarm health, diagnose job queue bottlenecks, and identify resource-heavy tenants.

## Research Report
- **Goal**: Unify the telemetry pipelines to ensure Cloud and Standalone modes emit identical, high-fidelity OpenTelemetry metrics for job queues, agent response latencies, and tenant resource usage.
- **Current State Analysis**:
  - The `telemetry.go` module currently tracks several critical metrics (e.g., `agent_execution_traces_total`, `sub_agent_queue_delay`, `bubblewrap_violation_total`).
  - However, there is a clear discrepancy in tracking network partition events, standalone SQLite IO latency, and agent memory limits.
  - The `BufferMetricFunc` fallback mechanism for PII-redacted metrics works, but Grafana dashboard visualization is non-existent for Standalone SQLite degraded metrics.
  - Per-tenant token burn rates are only partially tracked via `TokenBurnRatePredicted24h` without corresponding query metrics for the SQLite mode.
- **Key Bottlenecks Identified**:
  - Job queue depth builds up rapidly in Standalone mode due to local SQLite locks (`distributed_locks` vs `Redlock`), but the queue depth metric is not continuously sampled.
  - AutoDream sync engines occasionally fail silently without alerting on `autodream_sync_errors_total` when network connectivity is lost.

## Design Doc
1. **Unified Metric Instrumentation**: Implement a consistent `RecordQueueDepth` and `RecordDbLatency` hook that abstracts away the difference between Redis and SQLite implementations.
2. **Dashboard Synchronization**: Create a unified Grafana dashboard JSON model that dynamically switches panels based on the `deployment_mode` attribute (Cloud vs Standalone).
3. **Agent Swarm Health View**: Aggregate sub-agent failure rates and queue delays into a single "Swarm Health Score" metric.
4. **Tenant Cost Metering**: Expand the existing `TokenBurnRatePredicted24h` to include localized LLM inference fallback costs (e.g., MiniMax) for the Standalone mode.

## Implementation Prompt
Implement missing telemetry hooks in `srcs/server/telemetry/metrics.go` and `telemetry.go` to cover job queue depth, SQLite I/O latency, and sub-agent memory usage. Ensure all new metrics include a `deployment_mode` attribute to distinguish between Cloud and Standalone environments. Create the corresponding Grafana dashboard JSON definition in the `monitoring/` directory to visualize the "Swarm Health Score", tenant token burn rates, and queue bottlenecks.

## Priority
P1

## Estimated Scope
Medium

</div>
