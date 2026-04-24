# Swarm Agentic Operations Efficiency Report

## 1. Title
Hybrid Telemetry and Observability Gap Resolution for Agent Harness

## 2. Problem Statement
The OHC platform currently lacks comprehensive observability of agent execution latency and data context routing in its Grafana dashboards. While the telemetry module captures metrics like `HarnessExecutionLatency` and `OmniContextBytesRouted`, they are not visualized. This prevents swarm operators and business owners from identifying bottlenecks in sync conflicts between Cloud-native and Standalone modes and from assessing the efficiency of the Swarm.

## 3. Research Report
### Hybrid Telemetry Review
Analysis of `src/server/telemetry/telemetry.go` reveals that we are capturing `HarnessExecutionLatency`. Execution telemetry is currently underutilized in our Prometheus metric aggregations. Throughput data indicates that Standalone mode incurs different execution profiles compared to the Cloud's architecture. We also track `SyncConflictsResolvedTotal` and `OmniContextBytesRouted`.

### Observability Gap Analysis
Audit of the observability stack shows missing visual coverage for several key metrics:
- **Execution Latency:** `HarnessExecutionLatency` is absent, masking the actual agent reasoning time.
- **Data Context:** `OmniContextBytesRouted` is tracked in `telemetry.go` but not visualized, hiding data routing efficiency differences.
- **Sync Conflicts:** `SyncConflictsResolvedTotal` is recorded but lacks a dedicated dashboard panel.

### Bottleneck Hunting
Inefficiencies manifest differently across modes. In Cloud-native mode, `SyncConflictsResolvedTotal` can indicate primary bottlenecks due to coordination across multiple worker pods. In Standalone mode, the primary bottleneck is single-node compute limits during large agentic bursts, reflected in `HarnessExecutionLatency`. Another bottleneck observed is related to `AutoDreamIngestionError` during data ingestion.

### Swarm Health Assessment
The swarm is generally healthy, but we need to track `RagEscalationCount` to assess stability metrics. Periodic spikes in escalations or `AutoDreamIngestionError` occurrences indicate some agents attempting out-of-scope actions or failing to process context.

### Cost Efficiency Analysis
Per-tenant cost metering can be inferred via `OmniContextBytesRouted`. We observe anomalously high data usage for certain tenants doing bulk automated tasks without batching. Currently, this data is exported but not effectively aggregated in a "Business Advisory" style report. Optimizing the byte routing could reduce costs for routine operations.

## 4. Design Doc
**High-Level Architecture & Integrations:**
- Update Grafana dashboards to include three new panels:
  1. **Harness Execution Latency:** A timeseries panel plotting `HarnessExecutionLatency`.
  2. **Sync Conflicts:** A bar gauge or timeseries showing `SyncConflictsResolvedTotal`.
  3. **Context Byte Routing:** A table or bar chart grouping `OmniContextBytesRouted`.
- Maintain OHC premium visual standards (Glassmorphism, Outfit/Inter typography) by reusing the existing OHC Aesthetic Injector HTML panel.
- No changes to DB schema or API definitions are necessary, as the Go telemetry module already instruments these metrics.

## 5. Implementation Prompt
Please update the Grafana dashboards to include panels for `HarnessExecutionLatency`, `SyncConflictsResolvedTotal`, and `OmniContextBytesRouted`. Ensure the visualizations clearly differentiate between Cloud and Standalone modes where applicable, and adhere to the OHC Premium Token library design system. Create comprehensive E2E tests verifying that the dashboard successfully loads these new panels.

## 6. Priority
P1

## 7. Estimated Scope
Medium
