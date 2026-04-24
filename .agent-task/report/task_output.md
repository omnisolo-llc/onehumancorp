# Hybrid Swarm Health & Observability Review

## Problem Statement
The OHC platform operates across two primary deployment modes: a Cloud-native multi-tenant Kubernetes environment backed by PostgreSQL, and a Standalone local desktop mode utilizing SQLite. While the platform currently aggregates top-level API latency and token burn rates, there are severe blind spots regarding how AI agent Swarm queues process background tasks in these distinct modes. Specifically, in Standalone mode, SQLite's concurrency limitations lead to significant lock contention (`sqliteLockContentionCounter`) and subsequent Swarm queue backlogs, whereas PostgreSQL's `SKIP LOCKED` behavior efficiently processes queues. Non-technical business owners and AI Swarm Operators currently lack human-readable diagnostics and actionable insights regarding these mode-specific operational inefficiencies.

## Research Report
Based on the execution data analysis of Kubernetes logs, standalone instances, and the existing observability stack in `srcs/server/telemetry`:

1. **Hybrid Telemetry Gaps**:
   - Many critical background metrics (such as queue processing time) lack the `deployment_mode` attribute, meaning operators cannot filter data effectively between Cloud and Standalone environments.
   - Specifically, metrics for AutoDream task consolidation (queue depth, job latency) and general Swarm task claim contention lack granular dimension tagging.

2. **Dashboard Deficiencies**:
   - We observed dashboards such as `monitoring/dashboards/harness_efficiency.json` and `kairos_dashboard.json` do support mode-based visualization for initialization and IO latency.
   - However, they completely omit sections tracking AutoDream consolidation wait times, Swarm dead-letter queue sizes, or specific entity-level transaction latency (e.g., how long it takes to process a `booking` vs. an `order`).

3. **Bottlenecks (Cloud vs. Standalone)**:
   - Standalone: Rapid, concurrent Swarm tasks (e.g. syncing Teammate Mesh, running AutoDream memory compression) trigger widespread lock retry loops in SQLite, vastly increasing job latency.
   - Cloud-native: High tenant volume leads to occasional queue depth spikes but without the DB-level contention blockages seen locally.

4. **Agent Context**:
   - The Business Advisory ("The Advisor") agent currently has no context on queue health. It cannot issue plain-language alerts such as: "Your automated operations are currently delayed due to high system load. Things will catch up shortly."

## Design Doc

To address these shortcomings, we must deploy a unified Hybrid Telemetry framework that correctly dimension-tags background queue processes and exposes them via new dashboards.

- **Unified Mode Tagging**: All Swarm and AutoDream metrics must consistently apply the `deployment_mode` label.
- **New Core Metrics (OpenTelemetry)**:
   - `ohc_swarm_job_latency_by_entity_seconds`: Histogram tracking processing time broken down by `mode` and target `entity` (products, orders, bookings).
   - `ohc_autodream_queue_depth`: Gauge tracking pending memory consolidation tasks (Cloud vs Standalone).
   - `ohc_task_claim_contention_total`: Counter for lock retry and failed claim events (already initialized in `telemetry.go` but not consistently implemented across workers).
- **Dashboard Configurations**:
   - Create `monitoring/dashboards/hybrid_ops_dashboard.json` and `monitoring/dashboards/hybrid_swarm_cost_analytics.json`.
   - Panels will directly map Cloud Postgres lock contention against Standalone SQLite retries, and measure API call costs attributed to specific queue entities.

## Implementation Prompt
Update the `srcs/server/telemetry/telemetry.go` file to fully initialize the `AutoDreamQueueDepth` gauge and `SwarmJobLatencyByEntitySeconds` histogram with `deployment_mode` attributes. Apply these metrics into the background worker implementations in `srcs/server/agents/kairos/` and `srcs/server/pipeline/` ensuring that both PostgreSQL and SQLite queue fetches are metered accurately. Finally, create a new Grafana JSON dashboard located at `monitoring/dashboards/hybrid_swarm_health.json` that plots AutoDream queue depth, job processing latency by entity, and task claim contention, separated by deployment mode. Ensure the Business Advisory prompt context layer is updated to accept summary health strings from these new metrics to alert users in simple terms if processing stalls. Write end-to-end tests simulating SQLite contention bursts to verify metric generation.

## Priority
P1

## Estimated Scope
Medium
