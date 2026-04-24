# Telemetry Insights: Cloud vs Standalone Modes

## Title
Telemetry Analysis & Hybrid Sync Optimization

## Problem Statement
The OHC platform operates in a hybrid environment where Standalone instances synchronize telemetry and agent state back to the Cloud nodes. The current batching mechanism (`SyncBufferedMetrics`) does not dynamically adapt to network conditions, leading to inefficient payload sizes and timeouts. Additionally, observability gaps exist around synchronization error tracking and backoff duration monitoring.

## Research Report
- **Throughput & Latency Differences:** Cloud-native nodes directly expose metrics via Prometheus exporters resulting in near-zero aggregation latency. Standalone nodes buffer telemetry locally (`telemetry_buffer`) and utilize background sync workers. Under high agent load or network partitions, local queues deepen and subsequent batch syncs (`SyncBufferedMetrics`) experience high latencies.
- **Bottlenecks:**
  - *Hardcoded Batch Size:* In `SyncBufferedMetrics` (and `StartSyncDaemon`), batch sizes do not adjust smoothly to variable payload complexity.
  - *Error Rate Visibility:* When sync fails, retries occur but specific HTTP status codes and transient failure causes are aggregated poorly.
- **Observability Gap Analysis:** Missing metric coverage for explicit tracking of synchronization failure reasons (e.g. timeout vs 429 vs 500) and actual sync processing delays at the Orchestrator (SIP DB) level. Cost-efficiency per tenant is currently difficult to track as payload size and sync frequency aren't well correlated.

## Design Doc
1. **Dynamic Batch Sizing:** Update the synchronization daemon (`StartSyncDaemon`) to implement an adaptive batch size based on historical latency and error rates, moving away from fixed defaults.
2. **Enhanced Observability:** Add detailed Prometheus metrics (`ohc_sync_error_total` by reason, `ohc_sync_batch_duration`) in the `telemetry` package to gain visibility into the hybrid sync process.
3. **Queue Health Alerts:** Implement Grafana panels visualizing the depth of the local `telemetry_buffer` and tracking `ohc_token_budget_alert_total` for high-volume tenants.

## Implementation Prompt
"Enhance the telemetry synchronization worker to dynamically adjust batch sizes based on network conditions and error rates. Add OpenTelemetry metrics for sync failure reasons and buffer queue depth. Update `SyncBufferedMetrics` in `sip.go` to return more granular errors."

## Priority
P2

## Estimated Scope
Medium
