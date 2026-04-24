# [Metrics] AutoDream Observability Discrepancies & Gap Analysis

## Title
AutoDream Observability & Missing Metric Definitions

## Problem Statement
The KAIROS AI OS Hybrid Architecture relies heavily on the "AutoDream" mechanism to synchronize, compress, and ingest memories across Cloud and Standalone environments. However, a review of the global telemetry definitions (`src/server/telemetry/telemetry.go`) versus the Grafana dashboard configurations (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) reveals discrepancies. The Grafana dashboards query metrics like `autodream_memories_ingested_total` and `autodream_memories_compressed_total`, but the telemetry package registers them as `ohc_autodream_memories_ingested_total` and `ohc_autodream_memories_compressed_total`. Additionally, several critical AutoDream telemetry signals such as `ohc_autodream_consolidation_total` and sync error states do not have visual representation in any Grafana dashboards. This inconsistency obscures the true health of the AutoDream memory pipeline from swarm operators and the human CEO, making it difficult to debug hybrid memory synchronization failures.

## Research Report
- **Goal**: Audit and resolve telemetry metric mismatch and improve observability for the AutoDream memory pipeline across the Hybrid Architecture.
- **Findings**:
  - The telemetry variables `AutoDreamMemoriesIngestedCounter` and `AutoDreamMemoriesCompressedCounter` are correctly registered in `telemetry.go` as `ohc_autodream_memories_ingested_total` and `ohc_autodream_memories_compressed_total`.
  - The Grafana dashboard (`hybrid-telemetry.json`) incorrectly references them in some panels as `autodream_memories_ingested_total` and `autodream_memories_compressed_total` (missing the `ohc_` prefix), leading to broken visualizations ("No Data" panels).
  - Important error metrics like `autodream_sync_errors_total`, `ohc_autodream_ingestion_error_total`, and `ohc_autodream_compression_error_total` are recorded in the code but entirely omitted from the "AutoDream Memory Pipeline" section of the dashboard, making it impossible to diagnose silent ingestion failures.
  - Consolidation cycles (`ohc_autodream_consolidation_total`) lack dashboard visibility.
- **Impact**: Without accurate metrics tracking, operators cannot distinguish between a quiet system and a failing AutoDream background worker, leading to potential catastrophic loss of agent contextual memory in Standalone nodes.

## Design Doc
1. **Dashboard Update (`hybrid-telemetry.json`)**:
   - Update existing metric queries in the "AutoDream Memory Pipeline" panel to use the correct `ohc_` prefix.
   - Add new Time Series / Stat panels to visualize error rates (`autodream_sync_errors_total`, `ohc_autodream_ingestion_error_total`, `ohc_autodream_compression_error_total`).
   - Add a panel for `ohc_autodream_consolidation_total` to track successful background consolidation cycles.
2. **Telemetry Code Audit (`src/server/telemetry/telemetry.go`)**:
   - Standardize metric naming. Ensure all AutoDream metrics begin consistently with `ohc_autodream_`. Rename `autodream_records_synced_total` to `ohc_autodream_records_synced_total` and `autodream_sync_errors_total` to `ohc_autodream_sync_errors_total` for consistency. Update all callsites and queries.

## Implementation Prompt
Update the `src/server/telemetry/telemetry.go` to standardize all AutoDream metric names to include the `ohc_autodream_` prefix (specifically fixing `autodream_records_synced_total` and `autodream_sync_errors_total`). Then, update the `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` dashboard to correctly reference the `ohc_` prefixed metrics, and add new visualization panels for AutoDream sync errors, ingestion errors, compression errors, and consolidation cycles to give operators full visibility into the memory pipeline.

## Priority
P1

## Estimated Scope
Small
