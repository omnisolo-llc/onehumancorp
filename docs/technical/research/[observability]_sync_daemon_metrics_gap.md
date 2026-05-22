# Title: Add Mode-Specific Prometheus Metrics to Hybrid MCP RAG Sync Daemon

## Problem Statement
The Hybrid MCP RAG Sync Daemon (`HybridMCPRAGDaemon` in `src/server/orchestration/hybrid_sync daemon`) synchronizes local agent missions to the cloud in Standalone mode. While it currently emits metrics via `telemetry.RecordSyncEscalation`, `RecordSyncDaemonBatchSize`, `RecordSyncLatency`, and `RecordSyncPayloadSize`, these telemetry calls do not distinguish between execution contexts or capture detailed mode-specific throughput failures and error rates. Additionally, the corresponding Grafana dashboard (`kairos_hybrid_metrics.json`) lacks visualization for sync operations, creating an observability gap for Standalone client sync reliability.

## Research Report
An audit of `src/server/orchestration/hybrid_sync daemon` shows that `ProcessSync` processes batches of up to 500 `agent_missions`. Although `telemetry.Record*` methods are invoked, they are generic wrappers. To satisfy OHC's Full-Spectrum Observability requirement, we need detailed Prometheus metrics specifically categorized by hybrid modes (e.g., Standalone SQLite vs Cloud API fallback), capturing specific error rates (e.g., API timeouts vs DB lock errors). Grafana dashboards like `kairos_hybrid_metrics.json` must be updated to display these critical bottleneck indicators.

## Design Doc
1. Define Prometheus metrics in `src/server/telemetry` or the specific daemon package for sync throughput, latency (Histogram), and error rates (Counter), tagged with a `mode` label.
2. Update `src/server/orchestration/hybrid_sync daemon` to increment these mode-labeled metrics upon success or failure of `ProcessSync`.
3. Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to include panels for:
   - Sync Error Rate by Mode
   - Sync Latency (P95) by Mode
   - Sync Payload Size and Batch Depth
4. Ensure the UI panels apply the OHC Premium Glassmorphism styling natively inside Grafana's Text/HTML panels.

## Implementation Prompt
You are an Implementer. Implement the design above:
1. Identify the Prometheus `telemetry` methods used in `hybrid_sync daemon` and ensure they support and record a `mode` label (e.g., `Standalone` vs `Cloud`).
2. Add a `SyncDaemonErrorTotal` Counter in the `telemetry` package and increment it when `ProcessSync` fails.
3. Add three new panels to `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to visualize the sync latency (P95), error rate, and batch size, querying the updated Prometheus metrics with mode labels.
4. Verify tests pass using `bazel test //...`.

## Priority
P1

## Estimated Scope
Medium
