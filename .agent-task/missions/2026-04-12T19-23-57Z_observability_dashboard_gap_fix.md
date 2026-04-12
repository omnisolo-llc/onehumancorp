---
status: DONE
agent: jules
priority: P1
---

# Title: Add Missing Hybrid Telemetry Dashboards for AutoDream, LLM Cache & Sync Daemon

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) currently exhibits significant Grafana visualization gaps. A comprehensive audit of `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` revealed that critical role-specific and architectural inefficiencies are being masked. Specifically, while `srcs/server/telemetry/telemetry.go` tracks metrics for AutoDream (RAG performance), LLM Cache hit/miss rates, and Cloud-to-Standalone Sync Daemon behaviour, these metrics have no corresponding panels in Grafana. This prevents proactive bottleneck hunting between Cloud and Standalone modes.

## Research Report
- **AutoDream / RAG Performance:** Metrics like `ohc_autodream_sync_duration_seconds`, `ohc_autodream_query_duration_seconds`, `autodream_memories_ingested_total`, and `autodream_memories_compressed_total` are instrumented but invisible. Cloud vs Standalone RAG inefficiencies cannot be compared.
- **LLM Cache Efficiency:** `ohc_cache_hits_total` and `ohc_cache_misses_total` are tracked but invisible, making it impossible to analyze caching effectiveness for AI tasks.
- **Standalone Sync Stability:** `ohc_sync_escalations_total`, `ohc_sync_latency_seconds`, `ohc_sync_payload_size_bytes`, and `ohc_sync_daemon_batch_size` are recorded during local-to-cloud synchronization but not visualized, hiding Sync Daemon batching performance in Standalone mode.

## Design Doc
1. **Update Grafana Dashboards:** Modify `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include the following new panels:
   - **AutoDream Memory Pipeline:** Line charts or bar charts tracking `autodream_memories_ingested_total` and `autodream_memories_compressed_total` grouped by `agent_id`.
   - **AutoDream RAG Latency:** Histogram or timeseries tracking the 95th percentile of `ohc_autodream_sync_duration_seconds` and `ohc_autodream_query_duration_seconds` grouped by `deployment_mode`.
   - **LLM Cache Efficiency:** Panels tracking `ohc_cache_hits_total` and `ohc_cache_misses_total` grouped by `operation` and `cache_type`.
   - **Sync Daemon Health:** Panels showing `ohc_sync_daemon_batch_size`, `ohc_sync_latency_seconds`, and `ohc_sync_escalations_total`.
2. **Aesthetic Excellence:** Ensure any text or specialized UI elements within the dashboard leverage the OHC-SIP (Stylistic Intent Profile) using Glassmorphism CSS, 20px blur, and Outfit/Inter typography.

## Implementation Prompt
Hello Implementer, please execute the following task:
1. **Locate Target File:** Open `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. **Inject Missing Panels:** Add the following JSON panel definitions matching Grafana's timeseries and histogram schema:
   - A panel for AutoDream Latencies targeting `ohc_autodream_sync_duration_seconds` and `ohc_autodream_query_duration_seconds`.
   - A panel for AutoDream Memory Ingestion/Compression targeting `autodream_memories_ingested_total` and `autodream_memories_compressed_total`.
   - A panel for LLM Cache metrics targeting `ohc_cache_hits_total` and `ohc_cache_misses_total`.
   - A panel for Sync Daemon performance targeting `ohc_sync_latency_seconds`, `ohc_sync_daemon_batch_size`, and `ohc_sync_escalations_total`.
3. **Verify:** Use `grep` or `cat` to ensure the JSON remains perfectly formatted and valid. If the dashboard fails to load locally, correct the JSON schema.
4. **Testing:** The primary changes are configuration-based. Run standard pre-commit checks and submit the PR. No backend code modifications are needed unless an instrumentation bug is found during testing.

## Priority
P1

## Estimated Scope
Medium
