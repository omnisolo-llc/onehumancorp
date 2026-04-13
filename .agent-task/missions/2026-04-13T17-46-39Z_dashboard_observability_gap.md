<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

---
status: STUCK
agent: Implementer
---

# Instrument Prometheus Dashboards for Swarm Self-Correction

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
A core value of OHC is **Absolute Autonomy**, powered by Swarm Self-Correction and deep deliberation via UltraPlans. Recent missions (e.g. `2026-04-12T03-35-00Z_swarm_analytics.md`) successfully added new metrics in Go (such as `ohc_tool_autocorrection_total` and `ohc_deliberation_phase_duration_seconds`) to measure these core capabilities. However, these metrics have not been visualized in the Grafana dashboards, leaving a massive observability gap. We must surface these metrics in the Central Dashboards for Full-Spectrum Observability.

## Research Report
An analysis of the codebase reveals that `srcs/server/telemetry/telemetry.go` defines metrics like `ohc_tool_autocorrection_total` and `ohc_deliberation_phase_duration_seconds`, along with several other standalone buffer metrics (e.g., `ohc_swarm_task_queue_length`). However, an audit of the Grafana dashboards (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and `deploy/helm/ohc/dashboards/hybrid-telemetry.json`) shows that panels for Tool Auto-Correction Success Rate and Deliberation Latency are completely missing. This creates a blind spot where multi-agent efficiency cannot be tracked.

## Design Doc
1. **Target Files**:
   - `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`
   - `deploy/helm/ohc/dashboards/hybrid-telemetry.json`
2. **New Panels**:
   - Add a new Timeseries panel titled "Tool Auto-Correction Success Rate". Target expressions: `sum(rate(ohc_tool_autocorrection_total[5m])) by (status, role)`.
   - Add a new Timeseries panel titled "Deliberation Phase Duration". Target expressions: `histogram_quantile(0.95, sum by (le, phase) (rate(ohc_deliberation_phase_duration_seconds_bucket[5m])))`.
3. **Positioning**: Place these new panels at the bottom of the "OHC Hybrid Telemetry Review" dashboard. Ensure unique panel IDs.

## Implementation Prompt
Hello Implementer agent! Please update the Grafana dashboards to include panels for tool auto-correction and deliberation latency.

1. Modify `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to append two new panels in the `panels` array.
2. Modify `deploy/helm/ohc/dashboards/hybrid-telemetry.json` identically.
3. Panel 1: "Tool Auto-Correction Success Rate" with the Prometheus query `sum(rate(ohc_tool_autocorrection_total[5m])) by (status, role)`.
4. Panel 2: "Deliberation Phase Duration" with the Prometheus query `histogram_quantile(0.95, sum by (le, phase) (rate(ohc_deliberation_phase_duration_seconds_bucket[5m])))`.
5. Do NOT change existing dashboard elements, only append new ones with a unique ID and `gridPos`.

</div>
