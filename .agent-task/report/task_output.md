# OHC Harness Efficiency Observability Analysis

## Overview
As the Principal Data Scientist - Agentic Operations, an investigation was conducted into the Swarm observability specifically concerning Harness Efficiency across Cloud and Standalone modes. The goal was to identify missing telemetry metrics, visualization gaps, and propose actionable steps to enhance the Swarm operators' visibility into agent execution performance.

## Findings
1. **Instrumented Metrics**:
   - `HarnessInitLatency`
   - `HarnessDbIoLatency`
   - `HarnessExecutionLatency`
   All of these metrics have been successfully implemented and are actively recorded within `src/server/telemetry/telemetry.go` as Prometheus Histograms.

2. **Dashboard Visualization Gap**:
   - The Grafana dashboard located at `monitoring/dashboards/harness_efficiency.json` currently surfaces `HarnessInitLatency` and `HarnessDbIoLatency`.
   - The key metric, `HarnessExecutionLatency`, which tracks the actual task execution time per mode, is entirely missing from the dashboard panels.

## Impact
Without `HarnessExecutionLatency` visualized alongside the initialization and database I/O latencies, Swarm operators lack a holistic view of the agent's end-to-end execution. This blind spot prevents identifying whether slow agent responses for users (such as Maya or Carlos) are caused by initialization overhead, database sluggishness, or purely slow execution within the specific environment mode (Cloud vs. Standalone).

## Action Taken
An issue brief has been formally structured and committed to the research documentation directory:
- **Location**: `docs/research/[observability]_harness_efficiency_analysis.md`
- **Recommended Implementation**: Update `harness_efficiency.json` to include a new panel for `Harness Execution Latency (P95)` utilizing the PromQL query `histogram_quantile(0.95, sum(rate(harness_execution_latency_bucket[5m])) by (le, deployment_mode))`.

## Conclusion
Adding the missing panel to the Harness Efficiency dashboard will close the observability gap and ensure the platform fully adheres to the Core Values of "Full-Spectrum Observability," allowing for immediate identification of mode-specific execution bottlenecks.
