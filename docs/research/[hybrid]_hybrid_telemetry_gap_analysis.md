# OHC Hybrid Telemetry Gap Analysis & Bottlenecks

## Title
Hybrid Telemetry Review & Observability Gap Analysis

## Problem Statement
While the "Harness Efficiency" dashboard visualizes Agent Harness performance metrics, we must rigorously analyze these metrics across both Cloud-native (multi-tenant) and Standalone (local) deployment modes. The system currently uses environment variables (`OHC_MULTITENANT`, `OHC_STANDALONE`) to determine the mode. There are gaps in observability where some operations don't record the `deployment_mode` attribute, and there are discrepancies in how latency is captured between modes, making it difficult for Swarm operators to identify and remediate bottlenecks specific to the Standalone or Cloud environments. We need a unified telemetry review to ensure parity and visibility.

## Research Report
- **Goal**: Review telemetry data and identify throughput/error rate differences between Cloud and Standalone modes. Uncover observability gaps and identify performance bottlenecks.
- **Findings (Metrics & State Tracking)**:
  - The `RecordHarnessInitLatency`, `RecordHarnessDbIoLatency`, and `RecordHarnessExecutionLatency` metrics correctly accept a `mode` parameter.
  - However, the mode detection logic is scattered across the codebase. For example, in `srcs/server/harness/manager.go` and `srcs/server/agents/mcp/proxy/proxy.go`, `os.Getenv("OHC_STANDALONE") == "true"` is explicitly checked. This is an anti-pattern as per our memory guidelines ("Standalone mode is determined strictly by evaluating the multitenant flag (e.g., using `!envBoolDefault("OHC_MULTITENANT", true)` in Go)").
  - **Bottleneck 1 (Harness DB I/O)**: In Standalone mode (SQLite), concurrent DB I/O from multiple sub-agents can lead to lock contention, increasing `HarnessDbIoLatency` significantly compared to the Cloud Postgres RLS setup. This is a known issue but requires better visualization.
  - **Bottleneck 2 (Job Queue Depth)**: The `RecordSubAgentQueueDelay` metric does not differentiate by `deployment_mode`. A delay of 500ms in Cloud might be normal due to network routing, but unacceptable in Standalone where components are co-located.
  - **Bottleneck 3 (Task Claim Contention)**: The `RecordTaskClaimContention` metric exists but is not visualized on the Harness Efficiency dashboard. High contention indicates inefficient locking (e.g., fallback Mutex in SQLite vs PG `SKIP LOCKED`).
- **Observability Gaps**:
  - `SubAgentQueueDelayHistogram` lacks the `deployment_mode` attribute.
  - `TaskClaimContentionTotal` lacks visualization.
  - Cost metering (`AgentCostEstimateUSD`) doesn't reflect the "zero marginal cost" of local inference in Standalone mode.

## Design Doc
1. **Refactoring Mode Detection**:
   - Standardize deployment mode detection across all telemetry recording sites. Create a helper function in the `telemetry` package (or `dashboard` where it's already used) to reliably determine the mode based on `OHC_MULTITENANT`.
2. **Metric Enrichment**:
   - Update `RecordSubAgentQueueDelay` and `SubAgentQueueDelayHistogram` to include the `deployment_mode` attribute.
3. **Dashboard Enhancements**:
   - Add new panels to `harness_efficiency.json` or create a new dashboard (`hybrid_bottlenecks.json`) to visualize:
     - Sub-Agent Queue Delay by Mode (P95).
     - Task Claim Contention Rate by Mode.
     - DB I/O Latency outliers (focusing on SQLite lock contention).

## Implementation Prompt
"Standardize the deployment mode detection logic across the backend, specifically in `harness/manager.go` and `agents/mcp/proxy/proxy.go`, replacing direct `OHC_STANDALONE` environment variable checks with the authoritative `!envBoolDefault("OHC_MULTITENANT", true)` logic. Update the `telemetry` package to add the `deployment_mode` attribute to the `SubAgentQueueDelayHistogram` metric. Finally, enhance the Grafana dashboard configuration (`monitoring/dashboards/harness_efficiency.json`) to include panels for 'Sub-Agent Queue Delay (P95)' and 'Task Claim Contention Rate', segmented by deployment mode."

## Priority
P1

## Estimated Scope
Medium
