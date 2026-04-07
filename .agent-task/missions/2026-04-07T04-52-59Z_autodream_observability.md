---
status: PENDING
agent: ""
---
# Title: AutoDream Pipeline Observability Gap

## Problem Statement
The KAIROS Orchestrator's AutoDream Pipeline is responsible for critical long-term memory consolidation, synchronizing and vectorizing agent session data. However, our recent telemetry audit reveals a significant observability gap: the AutoDream metrics exposed in `srcs/server/telemetry/telemetry.go` (such as `ohc_autodream_memories_ingested_total`, `ohc_autodream_memories_compressed_total`, `ohc_autodream_sync_duration_seconds`, and `ohc_autodream_query_duration_seconds`) are not currently visualized in our Grafana dashboards. This blinds us to potential bottlenecks in Swarm intelligence synchronization across Cloud and Standalone modes.

## Research Report
1. **Metrics Audit:** The file `srcs/server/telemetry/telemetry.go` successfully instruments several AutoDream-specific OpenTelemetry metrics.
2. **Dashboard Audit:** A review of `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and `deploy/docker/grafana/provisioning/dashboards/ohc-hybrid.json` confirms that zero panels currently query these AutoDream metrics.
3. **Visual Mandate:** OHC Visual Excellence Mandate requires "Premium" aesthetic styling (Glassmorphism, 20px blur, etc.) for our internal dashboards. The existing `ohc-hybrid.json` already employs an HTML/CSS injector panel to enforce this.

## Design Doc
We need to add new timeseries panels to `hybrid-telemetry.json` and `ohc-hybrid.json` to monitor the AutoDream pipeline.

*   **New Panels Required:**
    1.  **AutoDream Memories Ingested Rate:** `sum(rate(ohc_autodream_memories_ingested_total[5m])) by (agent_id)`
    2.  **AutoDream Memories Compressed Rate:** `sum(rate(ohc_autodream_memories_compressed_total[5m])) by (agent_id)`
    3.  **AutoDream Sync Duration (P95/Avg):** `rate(ohc_autodream_sync_duration_seconds_sum[5m]) / rate(ohc_autodream_sync_duration_seconds_count[5m])` (sliced by `deployment_mode`)
    4.  **AutoDream Query Duration:** `rate(ohc_autodream_query_duration_seconds_sum[5m]) / rate(ohc_autodream_query_duration_seconds_count[5m])` (sliced by `deployment_mode`)
*   **Styling:** Ensure the new panels fit the existing layout and inherit the transparent styling mandated by the OHC premium look (`"transparent": true`).

## Implementation Prompt
Dear Implementer Agent,
Please update the Grafana dashboards (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and `deploy/docker/grafana/provisioning/dashboards/ohc-hybrid.json`) to include visualizations for the AutoDream metrics.
1. Add panels for `ohc_autodream_memories_ingested_total`, `ohc_autodream_memories_compressed_total`, `ohc_autodream_sync_duration_seconds`, and `ohc_autodream_query_duration_seconds`.
2. Ensure you use the correct datasource UID (`Prometheus` or `prometheus` depending on the file's existing panels).
3. Set `"transparent": true` on the new panels in `ohc-hybrid.json` to adhere to the Glassmorphism visual mandate.
4. Verify the JSON syntax is correct.

## Priority
P1

## Estimated Scope
Small
