# Add Harness Execution Latency to Harness Efficiency Dashboard

**Title**: Add Harness Execution Latency to Harness Efficiency Dashboard

**Problem Statement**:
The `HarnessExecutionLatency` metric is being captured in Prometheus but is missing from the Harness Efficiency Grafana dashboard. This obscures critical performance differences between Cloud and Standalone modes for Agent Harness execution. From the perspective of a swarm operator, without this visibility, it's impossible to evaluate if the execution environment itself is introducing overhead.

**Research Report**:
Analysis of `src/server/telemetry/telemetry.go` reveals three core metrics for the agent harness lifecycle: `HarnessInitLatency`, `HarnessDbIoLatency`, and `HarnessExecutionLatency`. However, reviewing the dashboard configuration in `monitoring/dashboards/harness_efficiency.json` shows it only visualizes `HarnessInitLatency` and `HarnessDbIoLatency`. The missing metric, exposed to Prometheus as the `harness_execution_latency_bucket`, tracks the core execution duration and is a vital indicator of runtime efficiency.

**Design Doc**:
Update the `harness_efficiency.json` dashboard to include a new panel for `Harness Execution Latency (P95)` alongside the existing `Init` and `DB I/O` panels. This should be a time series visualization, consistent with the other two panels.

**Implementation Prompt**:
Add a new time series panel to the Grafana dashboard configuration (`monitoring/dashboards/harness_efficiency.json`) that visualizes the `harness_execution_latency_bucket` metric (P95 quantile) grouped by `deployment_mode`. Use an expression similar to the existing panels: `histogram_quantile(0.95, sum(rate(harness_execution_latency_bucket[5m])) by (le, deployment_mode))`. Position the panel appropriately on the grid, matching the aesthetic of the existing widgets.

**Priority**: P2

**Estimated Scope**: Small
