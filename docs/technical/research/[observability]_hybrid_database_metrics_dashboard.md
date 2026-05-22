# Title: Create Hybrid Database Metrics Dashboard for OHC Telemetry

## Problem Statement
The One Human Corp (OHC) backend codebase heavily instruments database query latencies, error rates, and SQLite-specific lock contention via OpenTelemetry/Prometheus (e.g., `db.client.operation.duration`, `db.client.operation.errors`, and custom `sqlite_lock_contention` metrics in `src/server/db/provider.rs`). However, there is no corresponding Grafana dashboard in `monitoring/dashboards` to visualize this critical telemetry. Without this visualization, operators cannot effectively monitor database bottlenecks or distinguish between Postgres (Cloud-Native Mode) and SQLite (Standalone Mode) performance degradation.

## Research Report
- **Context**: The OHC Hybrid Architecture (OHC-HA) operates in Cloud-Native (PostgreSQL) and Standalone Desktop (SQLite) modes.
- **Codebase Analysis**: The `src/server/db/provider.rs` exposes `db.client.operation.duration` (Histogram) and `db.client.operation.errors` (Counter). Additionally, `src/server/telemetry/telemetry/mod.rs` tracks `sqlite_lock_contention`.
- **Gap**: The `monitoring/dashboards` directory currently only contains `chaos_dashboard.json`. No database dashboards exist.
- **Goal**: Implement a visually premium Grafana dashboard to track these database metrics across modes, fulfilling the "Full-Spectrum Observability" core value.

## Design Doc
1. **File Location**: Create `monitoring/dashboards/database_metrics.json`.
2. **Dashboard Structure**:
   - **Row 1: Overview**: Total QPS (Queries Per Second) and Global Error Rate, split by database type (Postgres vs SQLite).
   - **Row 2: Latency**: P50, P90, P99 query latency distributions (`db.client.operation.duration`).
   - **Row 3: SQLite Specifics**: SQLite lock contention rate (`sqlite_lock_contention`).
3. **Visual Excellence**: All panels must use OHC premium CSS tokens. For text panels, inject global `<style>` blocks with:
   - `backdrop-filter: blur(20px) saturate(200%)`
   - `background: rgba(255, 255, 255, 0.03)`
   - `font-family: 'Outfit', 'Inter', sans-serif`
4. **Data Source**: Prometheus.

## Implementation Prompt
Create a new Grafana dashboard JSON file at `monitoring/dashboards/database_metrics.json`.
The dashboard must visualize the following Prometheus metrics exported by our Rust backend:
- `db_client_operation_duration_seconds_bucket` / `db_client_operation_duration_seconds_count`
- `db_client_operation_errors_total`
- `sqlite_lock_contention_total`

Ensure the JSON dashboard model includes:
1. A "Total Query Latency" panel (Heatmap or Time series).
2. An "Error Rate by Operation" panel.
3. A "SQLite Lock Contention" panel.

Crucially, inject a Text panel with HTML mode enabled that adds the global OHC Glassmorphism CSS styling to the dashboard body. Do not replace it with localized inline styles. Use this `<style>` block:
```html
<style>
  .grafana-app {
      backdrop-filter: blur(20px) saturate(200%) !important;
      background: rgba(255, 255, 255, 0.03) !important;
      font-family: 'Outfit', 'Inter', sans-serif !important;
  }
</style>
```

Finally, ensure the file is properly formatted JSON and verify its structure.

## Priority
P1

## Estimated Scope
Medium
