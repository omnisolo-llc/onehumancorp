# Task Output: Implement Hybrid Swarm Health and Cost Analytics Dashboards

## Problem Statement
The OneHumanCorp (OHC) platform currently scales seamlessly from Standalone (local SQLite) to Cloud-native (K8s, Postgres) deployments. While general telemetry metrics exist for overall token burn and API latencies, there is a critical observability gap at the business operation level. General telemetry lacks granular dimension tagging linking latency and costs to specific entity operations like processing orders, customer interactions, product updates, and bookings.

Because of this, Human CEOs and AI Swarm Operators cannot accurately pinpoint why a local agent stalls due to SQLite lock contention on a booking, or why Cloud-based product fulfillment incurs unusually high LLM retries and costs.

## Research Report Findings
1. **Hybrid Telemetry Shortcomings:** After reviewing `src/server/telemetry/telemetry.go` and `src/server/telemetry/metrics.go`, existing functions such as `RecordAgentCost` and `RecordApiCallCost` do not map to granular business object attributes (`products`, `orders`, `customers`, `bookings`). Thus, aggregate cost data cannot identify localized financial leaks.
2. **Mode-Specific Throughput Variances:** Standalone mode heavily relies on SQLite. When concurrent agents touch the same entity profile (e.g., customer calendar), they encounter SQLite retry exhaustion. Conversely, Cloud-native PostgreSQL smoothly processes volume via `SKIP LOCKED` queues. Current metrics (`sqliteLockContentionCounter`) exist but lack entity-based visibility.
3. **Missing Correlation Dashboards:** There are no Grafana dashboards correlating swarm task latency with mode-specific database lock contention to determine efficiency drops and cost-burn spikes on a per-entity basis.

## Proposed Next Steps

1. **Modify Existing Metric Instruments:**
    - Enrich `RecordAgentCost(ctx context.Context, agentID, organizationID, role, model, entity string, cost float64)` and `RecordApiCallCost(ctx context.Context, organizationID, entity string, cost float64)` in `src/server/telemetry/telemetry.go` to capture the target business `entity`.

2. **Introduce New Swarm Latency Metrics:**
    - Register a new OpenTelemetry Float64Histogram named `ohc_swarm_job_latency_by_entity_seconds`.
    - Provide an API hook `RecordSwarmJobLatencyByEntity(ctx context.Context, mode, entity string, latency float64)` to attach the deployment `mode` (Cloud vs. Standalone) and target `entity` attributes.

3. **Dashboard Configuration (Grafana):**
    - Construct the `hybrid_swarm_cost_analytics.json` dashboard at `src/server/monitoring/dashboards/`.
    - Build visual panels spanning **Swarm Throughput**, **Entity Latency** (by orders, bookings, customers, products), **Contention Heatmap** (mapping `sqliteLockContentionCounter`), and **Cost Analytics Breakdown** to accurately represent the added metric dimensions.

4. **Testing Enhancements:**
    - Adapt all references inside `src/server/telemetry/telemetry_test.go` to support the new `entity` arguments on the cost records.
    - Write robust assertions validating `ohc_swarm_job_latency_by_entity_seconds` instrumentation tags.