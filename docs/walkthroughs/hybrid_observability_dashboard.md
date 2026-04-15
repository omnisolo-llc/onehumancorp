<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Full-Spectrum Hybrid Observability Dashboard Walkthrough

Welcome to the interactive walkthrough for the Full-Spectrum Hybrid Observability Dashboard. The One Human Corp (OHC) architecture demands that every feature exposes high-fidelity metrics via OpenTelemetry and Prometheus, empowering agents and the Human CEO with complete visibility.

## Architecture Flow

```mermaid
graph TD
    A[Worker Agents / Services] -->|OpenTelemetry OTLP| B(OTel Collector)
    B -->|Prometheus Exporter| C[(Prometheus TSDB)]
    C -->|PromQL| D(Grafana Dashboards)
    C -->|Metrics API| E[Internal User-Facing Dashboards]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

## Dashboard Guidelines

### 1. Zero Secrets & Authentication
Dashboard access is secured via SPIFFE/SPIRE zero-trust principles. Do not embed static API keys in dashboard configurations.

### 2. Prometheus Histogram Queries
When defining Prometheus histogram queries for OHC Grafana dashboards, ensure raw `_bucket` metrics used in `histogram_quantile` are always properly aggregated by the `le` label using `sum()`. You must also include any labels used in the panel's legend in the `by (...)` clause to prevent collapsing metrics and breaking visualizations.

**Correct Example:**
```promql
histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket{job="ohc_backend"}[5m])) by (le, method, path))
```

**Incorrect Example (Will Collapse Metrics):**
```promql
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket{job="ohc_backend"}[5m]))
```

### 3. Distributed Tracing
Agents emit distributed traces that stitch together the lifecycle of a task from Local Standalone intelligence to the Multitenant Cloud. Each trace carries the current Mission ID and Agent ID context.

## Interactive API

To query raw metrics directly via the OHC Central Orchestrator, use the following interactive endpoint:

### Fetch Core Health Metrics
**GET** `/api/v1/observability/metrics`
- **Response**: `{"active_agents": 42, "pending_missions": 0, "avg_task_latency_ms": 120, "db_mode": "cloud"}`

</div>
