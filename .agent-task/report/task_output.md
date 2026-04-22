# Title: Consolidate and Optimize Agent Harness Telemetry across Cloud and Standalone Deployments

## Problem Statement
Swarm operators and administrators currently face difficulties in diagnosing throughput and error rate discrepancies between Cloud-native (multi-tenant K8s) and Standalone (local) deployments of the OHC platform. While we have basic metrics like `harness_init_latency`, `harness_db_io_latency`, and `task_claim_contention` tagged with `deployment_mode`, there is a lack of cohesive observability that ties these metrics directly to agent efficiency, job queue depth, and per-tenant cost bottlenecks. From the perspective of a swarm operator, it is challenging to identify when a specific tenant in Cloud mode is consuming disproportionate AI resources compared to Standalone mode, or when database I/O latency is causing agent job queue backlogs.

## Research Report
### Telemetry Review and Gap Analysis
1. **Existing Telemetry**:
   - The platform utilizes OpenTelemetry and Prometheus for backend metrics, with a Grafana dashboard named "Harness Efficiency" (`monitoring/dashboards/harness_efficiency.json`) that visualizes `harness_init_latency` and `harness_db_io_latency` P95 percentiles by `deployment_mode`.
   - Various metrics exist in `srcs/server/telemetry/metrics.go`, such as `RecordTaskClaimContention`, `RecordAgentExecutionTrace`, `RecordSubAgentQueueDelay`, and `RecordHarnessExecutionLatency`.
2. **Identified Discrepancies**:
   - **Throughput & Error Rates**: Cloud mode (multi-tenant K8s) handles higher concurrency but shows increased `task_claim_contention` compared to Standalone mode, likely due to PostgreSQL `SKIP LOCKED` behavior under load across many tenants.
   - **Bottlenecks**: `harness_db_io_latency` is a primary bottleneck in Cloud mode due to remote database network hops, whereas Standalone mode is bottlenecked by `harness_execution_latency` bounded by local compute.
   - **Visibility Gaps**: There are no unified dashboards that correlate `sub_agent_queue_delay` with `task_claim_contention` and `deployment_mode`. Additionally, per-tenant cost metering (AI token usage per agent department) is tracked but not effectively visualized alongside deployment-level performance metrics.

### Cost Efficiency Analysis
- Per-tenant resource usage analysis indicates that the lack of clear visualization for AI token consumption versus queue delay leads to inefficient resource allocation. Some tenants may monopolize the queue in Cloud mode without triggering immediate alerts.

## Design Doc
### High-Level Architecture
- **Metrics Aggregation**: Enhance the existing `telemetry.go` layer to automatically inject `tenant_id` (where applicable) and `deployment_mode` into all critical agent queue and execution metrics.
- **Unified Grafana Dashboards**: Create a new set of Grafana dashboards ("Swarm Health" and "Tenant Cost Efficiency") that combine:
  - Queue depth and delay (`sub_agent_queue_delay`).
  - Database contention (`task_claim_contention`).
  - AI Execution latency (`harness_execution_latency`).
- **Alerting Integration**: Configure Prometheus Alertmanager rules for sustained high `task_claim_contention` or abnormal `sub_agent_queue_delay` specific to Cloud vs Standalone modes.

### UI / UX Flow (Operator Dashboard)
- **Mobile UX Flow (375px first)**:
  - **Screen 1**: High-level Swarm Health Overview. Large status indicators for Cloud vs Standalone health.
  - **Screen 2**: Drill-down into "Bottlenecks" showing bar charts of `task_claim_contention` and queue delays.
  - **Screen 3**: "Tenant Insights" listing top 5 tenants by AI usage/cost, sorted by resource consumption.
- **Desktop Flow**: Side-by-side comparative views of Cloud vs Standalone metrics, with interactive timelines for tracing queue delays back to specific agent departments.

### AI Agent Integration Points
- **Business Advisory Agent**: Expose a summarized version of these metrics to the Business Advisory agent so it can notify the business owner if their specific tasks are delayed due to system load, translating technical queue depths into plain-language updates (e.g., "Your marketing tasks are taking a bit longer than usual today").

## Implementation Prompt
**User-Facing Outcome**:
Swarm operators will have a comprehensive, single-pane-of-glass dashboard to monitor, compare, and alert on the efficiency of the Agent Harness across Cloud and Standalone modes. They will be able to instantly identify if database I/O, task contention, or AI execution latency is the current bottleneck, and pinpoint anomalous tenant usage.

**Critical User Journey (CUJ)**:
1. Operator navigates to the "Swarm Health" dashboard in Grafana (or integrated internal tool).
2. Operator views a comparative breakdown of Cloud vs Standalone latency and queue depth.
3. Operator notices a spike in `task_claim_contention` in Cloud mode.
4. Operator clicks on the metric to drill down and identifies that 3 specific tenants are generating 80% of the queue load.
5. Operator adjusts rate limits or queue priorities for those tenants.

**Acceptance Criteria**:
- `deployment_mode` and `tenant_id` are consistently applied as tags to `sub_agent_queue_delay`, `task_claim_contention`, and AI execution metrics.
- A new "Swarm Health" Grafana dashboard is added to the `monitoring/dashboards` directory, capturing the CUJ.
- Prometheus alerts are defined for high queue delay and contention thresholds.
- The `Business Advisory` agent system prompt/tools are updated to query basic queue status for user-facing explanations.

## Priority
P1

## Estimated Scope
Medium
