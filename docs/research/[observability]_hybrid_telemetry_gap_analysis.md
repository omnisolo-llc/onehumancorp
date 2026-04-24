# Hybrid Telemetry & Observability Gap Analysis

## Problem Statement
The OHC platform operates across dual deployment modes: a multi-tenant Cloud environment (K8s, Postgres) and a Standalone mode (local SQLite). While there are isolated metrics tracking API requests, token burn rates, and KAIROS state machine transitions, there is a lack of comprehensive, mode-aware telemetry that enables direct performance and efficiency comparisons between the two environments. This visibility gap obscures latency bottlenecks in hybrid mesh communication, queue contention in standalone mode, and resource saturation across deployments, preventing swarm operators from effectively self-correcting or diagnosing mode-specific inefficiencies.

## Research Report
- **Goal**: Perform a gap analysis of the current observability stack, identify bottlenecks between Cloud and Standalone modes, and define a roadmap for unified hybrid telemetry.
- **Findings**:
  1. **Incomplete Metric Tagging**: While some metrics (like `ohc_kairos_transitions_total` and `ohc_agent_task_queue_depth`) tag the deployment `mode`, many critical system metrics (e.g., in `src/server/telemetry/metrics.go` and `src/server/telemetry/rag_sync_metrics.go`) lack the deployment mode dimension.
  2. **Dashboard Deficiencies**: Existing Grafana dashboards (e.g., `agent_audit_dashboard.json`) primarily focus on agent task status and execution but lack panels comparing Cloud vs. Standalone throughput, queue depth disparities, or hybrid network partition recovery rates.
  3. **Queue & Lock Contention Visibility**: The system lacks detailed histograms for job queue processing time segmented by mode, making it impossible to identify if SQLite lock contention in Standalone mode is causing significant delays compared to Postgres `SKIP LOCKED` in the Cloud.
  4. **Cost & Resource Metering**: Per-tenant cost metering (token burn rate) exists but lacks visibility into local resource usage (CPU/Memory) on Standalone desktop nodes, preventing accurate overall health assessments.

## Design Doc
1. **Unified Metric Enrichment**:
   - Introduce a global OpenTelemetry interceptor/middleware that automatically injects the `deployment_mode` attribute (derived via `kairos.GetMode()`) into all outgoing metrics, spans, and logs.
2. **Key Metric Additions**:
   - `ohc_queue_processing_duration_seconds` (Histogram, tags: `mode`, `queue_type`, `status`)
   - `ohc_hybrid_sync_latency_seconds` (Histogram, tags: `direction`)
   - `ohc_database_lock_contention_total` (Counter, tags: `mode`, `db_type`)
3. **Dashboard Enhancements**:
   - Create a new "Hybrid Operations Dashboard" in `src/server/monitoring/dashboards/` that juxtaposes Cloud vs. Standalone performance side-by-side.
   - Include panels for: Task Queue Depth (Cloud vs. Local), Average Agent Response Latency, and Error Rate by Mode.
4. **Architecture Integration**:
   - Update `src/server/telemetry/telemetry.go` to initialize these new mode-aware metrics.
   - Ensure the Standalone mode buffers these metrics locally and flushes them to the central observability cluster when connectivity is restored (offline-first telemetry).

## Implementation Prompt
"Implement the missing Hybrid Telemetry metrics identified in the gap analysis. Update the telemetry initialization in `src/server/telemetry/telemetry.go` to include the `ohc_queue_processing_duration_seconds` and `ohc_database_lock_contention_total` metrics with a `mode` label. Modify the existing metric recorders (e.g., in `rag_sync_metrics.go` and `metrics.go`) to accept and log the current execution mode using `kairos.GetMode()`. Finally, add a new Grafana dashboard JSON definition at `src/server/monitoring/dashboards/hybrid_ops_dashboard.json` that visualizes these new metrics, comparing Cloud and Standalone throughput."

## Priority
P1

## Estimated Scope
Medium
