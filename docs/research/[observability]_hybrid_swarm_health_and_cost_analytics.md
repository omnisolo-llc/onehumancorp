<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: Implement Hybrid Swarm Health and Cost Analytics Dashboards

## Problem Statement
The OneHumanCorp (OHC) platform scales from Standalone (local SQLite) to Cloud-native (K8s, Postgres) deployments. While basic telemetry for API latencies and overall token burn exists, we lack a comprehensive, mode-aware observability strategy that isolates efficiency issues per business entity and deployment type. Specifically, small business operations like processing product orders, managing customer inquiries, and handling bookings currently generate raw telemetry that isn't granularly linked to swarm task bottlenecks or tenant-level cost efficiency. This leaves the human CEO and AI Swarm Operators unable to accurately diagnose why a local agent managing bookings might stall due to SQLite lock contention, or why an agent fulfilling product orders in the Cloud is accumulating abnormally high costs due to repeated LLM inference retries.

## Research Report
**Hybrid Telemetry Review:**
- Analysis of `src/server/telemetry/telemetry.go` and `metrics.go` indicates that metrics like `ohc_agent_cost_estimate_usd` and `ohc_storage_cost_estimate_usd` are recorded via OpenTelemetry but lack sufficient dimension tagging to map back to specific business objects like **products**, **orders**, **customers**, or **bookings**.
- Throughput variances: In Cloud-native mode, the PostgreSQL `SKIP LOCKED` pattern handles concurrent job queues for high-volume transactions smoothly. Conversely, in Standalone mode, concurrent AI agents updating the same customer profile or booking calendar encounter SQLite retry exhaustion (`sqliteLockContentionCounter`), leading to increased job latency.
- Currently, there is no high-level dashboard juxtaposing swarm task queue depth against these localized bottlenecks or tracking specific per-tenant API call costs tied to operations like quoting an order or processing a booking.

**Observability Gap Analysis:**
- Missing `ohc_swarm_job_latency_by_entity_seconds` (Histogram) to measure processing time for core business domains (e.g., entity: `order`, `booking`, `customer`, `product`).
- Missing cost mapping: We need `AgentCostEstimateUSD` and `ApiCallCostEstimateUSD` to be enriched with attributes indicating the business workflow being executed.
- Missing an aggregate Grafana dashboard (Hybrid Swarm Health and Cost Analytics) to correlate mode-specific database contention (Postgres locks vs. SQLite retries) with swarm task latency and financial burn rate.

## Design Doc
**Architecture:**
- **Metric Definitions:**
  - Introduce `ohc_swarm_job_latency_by_entity_seconds` as a histogram measuring the end-to-end processing time of a background task, tagged by deployment `mode` and target `entity` (products, orders, customers, bookings).
  - Enrich existing `RecordAgentCost` and `RecordApiCallCost` functions to accept and attach tags for the corresponding business entity and workflow ID.
- **Data Source Integration:**
  - Update `src/server/telemetry/metrics.go` to add these new dimensions.
  - Implement periodic push/pull mechanisms that ensure local SQLite metrics are eventually synced to the centralized cloud Prometheus cluster, retaining their Standalone tags.
- **Dashboard Structure (Grafana):**
  - **Swarm Throughput Panel:** Side-by-side comparison of job queue processing rates for Cloud vs. Standalone.
  - **Entity Latency Panel:** Average latency for operations grouped by products, orders, customers, and bookings.
  - **Contention Heatmap:** Visualizing `sqliteLockContentionCounter` vs. Cloud DB locks across different swarm task types.
  - **Cost Analytics Panel:** Per-tenant and per-entity cost breakdown (`AgentCostEstimateUSD` per order/booking).

## Implementation Prompt
Update `src/server/telemetry/metrics.go` and `src/server/telemetry/telemetry.go` to include the new mode-aware and entity-aware dimensions. Register a new histogram `ohc_swarm_job_latency_by_entity_seconds` tagged by `mode` and `entity` (which must support values like 'products', 'orders', 'customers', and 'bookings'). Enhance the `RecordAgentCost` and `RecordApiCallCost` functions to accept the `entity` type. Finally, create a new Grafana dashboard JSON configuration located at `src/server/monitoring/dashboards/hybrid_swarm_cost_analytics.json` containing the designated panels for Swarm Throughput, Entity Latency, Contention Heatmap, and Cost Analytics. Ensure that all telemetry changes accurately log deployment mode (`Cloud` vs `Standalone`) using the established context patterns. Ensure an E2E test validates the new metric dimensions.

## Priority
P1

## Estimated Scope
Medium

</div>
