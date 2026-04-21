# [observability] Implement Grafana Dashboards for OHC Core Telemetry (Database, Queues, Sandbox)

## Title
Implement Missing Grafana Dashboards for Core OHC Telemetry

## Problem Statement
The One Human Corp (OHC) backend codebase heavily instruments critical operations such as database query latencies, SQLite-specific lock contention, sub-agent queue delays, and sandbox violations via OpenTelemetry/Prometheus (in `srcs/server/db/provider.go`, `srcs/server/telemetry/telemetry.go`, etc.). However, corresponding high-fidelity Grafana dashboards are missing in `srcs/server/monitoring/dashboards/` for some of these essential metrics.
Without visual representations, operators cannot effectively monitor database bottlenecks, distinguish between Postgres (Cloud-Native Mode) and SQLite (Standalone Mode) performance degradation, or analyze sub-agent queue depths and queue processing delays in real time. This violates the OHC "Full-Spectrum Observability" core value.

## Research Report
- **Context**: The OHC Hybrid Architecture (OHC-HA) operates in Cloud-Native (PostgreSQL + Redis) and Standalone Desktop (SQLite + Local Queue) modes.
- **Codebase Analysis**:
  - `srcs/server/db/provider.go` exposes `db.client.operation.duration` (Histogram) and `db.client.operation.errors` (Counter).
  - `srcs/server/telemetry/telemetry.go` tracks `ohc_sqlite_lock_contention_total`, `sub_agent_queue_delay`, `sandbox_violation_total`, `autodream_sync_errors_total`, etc.
  - The `srcs/server/monitoring/dashboards` directory contains `chaos_dashboard.json`, `agent_audit_dashboard.json`, and `kairos_dashboard.json`, but no database metrics or sub-agent queue metrics dashboards exist.
- **Gap Identification**:
  - Missing Database Dashboard: Visualizations for `db_client_operation_duration_seconds`, `db_client_operation_errors_total`, and `ohc_sqlite_lock_contention_total`.
  - Missing Queue Metrics: `ohc_agent_task_queue_depth` is in `kairos_dashboard.json`, but we lack deeper queue metrics visualizations such as `sub_agent_queue_delay` and `task_claim_contention_total` to trace bottlenecks.
- **Goal**: Create and extend Grafana dashboards to include these observability gaps. These dashboards must be visually premium, reflecting OHC's Glassmorphism design tokens.

## Design Doc
1. **File Locations**:
   - Create `srcs/server/monitoring/dashboards/database_metrics.json`.
   - Create `srcs/server/monitoring/dashboards/sub_agent_queue_metrics.json`.
2. **Dashboard Structures**:
   - **Database Dashboard**:
     - **Row 1: Overview**: Global Error Rate and Query counts (`db_client_operation_errors_total`).
     - **Row 2: Latency**: P50, P90, P99 query latency distributions (`db_client_operation_duration_seconds`).
     - **Row 3: SQLite Specifics**: SQLite lock contention rate (`ohc_sqlite_lock_contention_total`).
   - **Sub-Agent Queue Dashboard**:
     - **Row 1: Queue Depth & Delay**: Visualizing `sub_agent_queue_delay` histogram alongside `ohc_agent_task_queue_depth`.
     - **Row 2: Claim Contention**: Tracking `task_claim_contention_total` to monitor multi-node or standalone concurrency friction.
3. **Visual Excellence**: All panels must use OHC premium CSS tokens. For text panels, inject global `<style>` blocks with:
   - `backdrop-filter: blur(20px) saturate(200%)`
   - `background: rgba(255, 255, 255, 0.03)`
   - `font-family: 'Outfit', 'Inter', sans-serif`
4. **Data Source**: Prometheus.

## Implementation Prompt
1. Create a new Grafana dashboard JSON file at `srcs/server/monitoring/dashboards/database_metrics.json`.
The dashboard must visualize the following Prometheus metrics exported by our Go backend:
- `db_client_operation_duration_seconds_bucket` / `db_client_operation_duration_seconds_count`
- `db_client_operation_errors_total`
- `ohc_sqlite_lock_contention_total`

2. Create a new Grafana dashboard JSON file at `srcs/server/monitoring/dashboards/sub_agent_queue_metrics.json`.
The dashboard must visualize the following metrics:
- `sub_agent_queue_delay`
- `task_claim_contention_total`
- Sandbox violations using `sandbox_violation_total` to track agent harness stability.

Crucially, in each dashboard, inject a Text panel with HTML mode enabled that adds the global OHC Glassmorphism CSS styling to the dashboard body:
```html
<style>
  .grafana-app {
      backdrop-filter: blur(20px) saturate(200%) !important;
      background: rgba(255, 255, 255, 0.03) !important;
      font-family: 'Outfit', 'Inter', sans-serif !important;
  }
</style>
```

Verify that the files are properly formatted JSON files suitable for Grafana provisioning.

## Priority
P1

## Estimated Scope
Medium
