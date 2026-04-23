<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: Implement Missing Telemetry and Grafana Dashboards for Hybrid OHC Operations

## Problem Statement
While OneHumanCorp provides seamless scaling from a local Standalone desktop agent to a Cloud-Native K8s deployment, visibility into how our different AI agents and swarm processes operate across these environments remains fragmented. Certain business operations, especially multi-tenant token forecasting and hybrid job-queue throughput, lack corresponding high-fidelity visualizations. For operators and business owners relying on accurate financial and operational advisories, missing observability can mean hidden costs or undetected AI agent stalemates.

## Research Report
An audit of the `srcs/server/telemetry` layer and metric definitions in `telemetry.go` and `metrics.go` reveals that while core OpenTelemetry metrics exist (such as `TokenBurnRatePredicted24h`, `TaskQueueLengthGauge`, `SyncLatency`, and `TokenBudgetAlertTotal`), many are not actively synthesized into human-readable Grafana dashboards or plain-language Business Advisory insights.
Furthermore, the metrics gathered often show significant throughput variances: Cloud-Native modes tend to enqueue swarm tasks efficiently but can exhibit lock contention, whereas Standalone setups frequently run into SQLite retry exhaustion during rapid hybrid synchronization (`sqliteLockContentionCounter`).

## Design Doc
- **Telemetry Aggregation Framework**: Standardize exporting of OHC swarm state across K8s Prometheus endpoints and local standalone push gateways.
- **Dashboards Structure**:
  - **Swarm Health Dashboard**: Visualizing `swarmTaskQueueLengthGauge`, `taskEnqueuedCounter`, `taskFailedCounter`, and `swarmTaskProcessingLatency`.
  - **Financial Advisory Dashboard**: Tracking `AgentCostEstimateUSD`, `TokenBurnRatePredicted24h`, and `usdBurnRateGauge` with distinct per-tenant filtering.
  - **Hybrid Synchronization Dashboard**: Visualizing `LocalToCloudMissionSyncCount`, `AutoDreamRecordsSyncedTotal`, and mapping Cloud DB lock contention vs. Standalone SQLite retries.
- **AI Integration**: The Business Advisory ("The Advisor") department should ingest specific OpenTelemetry summaries via a new structured context layer to generate plain-language alerts (e.g., "Your AI costs are predicted to spike tomorrow. Do you want to pause automated quoting?").

## Implementation Prompt
Create the necessary Grafana dashboard configurations (JSON specs) for Swarm Health, Financial Advisory, and Hybrid Synchronization. Integrate missing OpenTelemetry counters where business actions aren't yet mapped, focusing specifically on local-to-cloud transition latency and cost threshold alerts. Ensure all additions strictly align with the `telemetry.InitWithMeter()` pattern in `srcs/server/telemetry.go`. Create an end-to-end test validating that simulated job queue depth correctly registers in Prometheus output and triggers a plain-language advisory warning through the Business Advisory department.

## Priority
P1

## Estimated Scope
Medium

</div>