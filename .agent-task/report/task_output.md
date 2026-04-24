# [Observability] Telemetry & Swarm Performance Review

## Problem Statement
The OneHumanCorp (OHC) platform operates in two distinct modes: Cloud-native (multi-tenant K8s) and Standalone (local desktop). Currently, there is a lack of deep observability into the differential performance characteristics of these two environments. Specifically, we need to understand how the AI agent swarm behaves, identify bottlenecks related to database contention (PostgreSQL vs. SQLite) and job queue depth, and ensure all critical business operations are adequately instrumented and visualized. Without this visibility, swarm operators cannot proactively self-correct or optimize efficiency for our diverse user base (from mobile-only bakers to local food carts).

## Research Report
### Hybrid Telemetry & Bottlenecks
A review of the `telemetry` module (`src/server/telemetry/telemetry.go`) and associated metrics reveals several key areas of instrumentation:
- **Lock Contention**: Metrics like `sqliteLockContentionCounter`, `sqliteRetryExhaustedCounter`, `postgresRetryExhaustedCounter`, and `sqliteThrottledRequestCounter` indicate a focus on understanding database locking issues, particularly in Standalone mode where SQLite is used. This suggests potential bottlenecks in concurrent agent operations on local deployments.
- **Mission Sync**: The `LocalToCloudMissionSyncCount` metric tracks the synchronization of missions from local to cloud environments. However, detailed latency metrics for this specific sync path or error rates during network partitions are less visible.
- **AI Agent Performance**: Metrics exist for token usage (`RecordTokenUsage`, `RecordAgentTokenUsage`) and specific API calls (e.g., Minimax). However, comprehensive tracking of agent response latency segmented by cloud vs. local execution is needed to identify environment-specific latency.

### Observability Gap Analysis
While technical metrics (HTTP latency, token count, lock contention) are well-represented, there's a gap in higher-level business operation observability.
- **Missing Metrics**: Metrics mapping directly to core business flows (e.g., "Time to publish website," "Quote generation latency," "Inventory update sync time") are either missing or not clearly aggregated into higher-level dashboards.
- **Dashboard Deficiencies**: The existing Grafana dashboards (in `monitoring/dashboards/`) need to be expanded to clearly delineate Cloud vs. Standalone performance, allowing operators to quickly identify if an issue is localized to a specific deployment mode.

### Swarm Health Assessment
The current instrumentation provides raw counts (e.g., `SyncEscalationsCount`), but lacks a cohesive "Swarm Health Score." We need metrics that capture the lifecycle of a mission: creation -> assignment -> execution -> completion/failure.
- Are missions getting stuck? (We need a metric for "mission time-in-queue").
- Is there resource contention? (The SQLite lock metrics are a good start, but need correlation with mission failure rates).

### Cost Efficiency Analysis
Per-tenant resource usage is partially trackable via `RecordAgentTokenUsage` (which includes `organizationID`). This is critical for identifying anomalous usage patterns. However, correlating this with storage usage or API call volume per tenant requires cross-referencing multiple metric sources, which should be unified in a dashboard.

## Design Doc
### Architecture Adjustments
1.  **Unified Swarm Health Metrics**: Introduce new metrics in `src/server/telemetry/telemetry.go` to track mission lifecycle states: `MissionQueuedLatency`, `MissionExecutionLatency`, and `MissionFailureRate` segmented by `tenant_id` and `deployment_mode`.
2.  **Business KPI Instrumentation**: Add custom metric recording functions for core business events (e.g., `RecordQuoteGenerated`, `RecordWebsitePublished`) to provide visibility into the success of agent departments.
3.  **Dashboard Enhancements**: Create two new primary Grafana dashboards:
    -   `Hybrid_Swarm_Health`: Side-by-side comparison of Cloud vs. Standalone mission throughput, latency, and database contention.
    -   `Tenant_Cost_Efficiency`: Aggregated view of token usage, storage, and API calls per tenant.

### Mobile UX Considerations
While telemetry is primarily for operators, exposing aggregated, plain-language insights to the user (e.g., via the "Business Advisory" agent) is crucial. Ensure metrics can be easily digested by the Business Advisory prompt to say, "Your AI agents handled 50 customer inquiries this week."

## Implementation Prompt
**Critical User Journey (CUJ):** A swarm operator needs to diagnose a sudden drop in successful quote generations by the Sales agent in Standalone deployments.

**Acceptance Criteria:**
1.  Add new metrics to `src/server/telemetry/telemetry.go` for `MissionTimeInQueue` (Histogram) and `BusinessEventCount` (Counter, with labels for event type).
2.  Instrument the `orchestration` module to record these new metrics when missions change state.
3.  Ensure all new metrics are properly exposed via the `/metrics` endpoint.
4.  Add a new E2E test that simulates a mission lifecycle and verifies the telemetry metrics are updated correctly (using a mock Prometheus scraper or internal metric inspection if available).
5.  All metric additions must adhere to the PII redaction standards (`RedactInterfacePII`).

## Priority
P1

## Estimated Scope
Medium
