# Research Report: Hybrid Telemetry & Observability Gap Analysis

## Problem Statement
The OHC Hybrid Orchestration system (KAIROS) runs in multiple modes (Cloud vs. Standalone) and requires comprehensive observability. However, there is a discrepancy in how Sub-Agent Task Queue depth is recorded. The Grafana dashboards (`kairos_hybrid_metrics.json`) visualize queue depth using `sum(ohc_agent_task_queue_depth) by (mode)`. While this metric is initialized in `src/server/orchestration/kairos/metrics.go`, the broader `src/server/telemetry` system uses an alternative OpenTelemetry metric `ohc.sub_agent.queue_length` (via `subAgentQueueLengthGauge`). This creates a fragmented observability experience. Furthermore, additional gaps such as missing visualizations for RAG escalations and mission costs highlight the need to consolidate metrics and enhance the dashboards.

## Research Findings
1. **Metric Fragmentation**: `ohc_agent_task_queue_depth` is a Prometheus metric maintained directly in the `kairos` package, whereas `ohc.sub_agent.queue_length` is an OpenTelemetry metric inside the `telemetry` module.
2. **Dashboard Misses**:
   - `kairos_hybrid_metrics.json` utilizes the `ohc_agent_task_queue_depth` Prometheus metric but relies on OpenTelemetry metrics elsewhere.
   - Observability gaps were found around token budget alerts, capability violations, RAG escalations, and mission costs (`ohc_token_budget_alert_total`, `capability_violation_total`, `ohc_rag_escalation_total`, `mission_cost_cents`), which lack complete visualization in the core dashboards.
3. **Execution Mode Support**: The metric label `mode` (cloud, standalone, headless) is essential for KAIROS, and it is explicitly supported by `ohc_agent_task_queue_depth` and transitions metrics, providing critical multi-tenant versus single-tenant execution insights.

## Proposed Action Plan
1. **Fix the Queue Depth Metric Mapping**: Deprecate the legacy `ohc.sub_agent.queue_length` if it's redundant or update the Grafana dashboard to track all queue measurements properly. In KAIROS, ensure the queue depth metric is consistently updated via `RecordTaskQueueLength` or a dedicated wrapper.
2. **Add Missing Panels**: Create new panels in `kairos_hybrid_metrics.json` or `agent_audit_dashboard.json` for RAG Escalation Rates, Token Budget Alerts, and Mission Costs per Tenant.
3. **Enhance KAIROS Traces**: Ensure that all KAIROS mode metrics effectively record latency and throughput differences between Cloud and Standalone modes.

## Priority
**P1 (High)**: Restoring queue depth visibility and adding critical cost/escalation panels are essential for swarm health assessment.

## Estimated Scope
Medium
