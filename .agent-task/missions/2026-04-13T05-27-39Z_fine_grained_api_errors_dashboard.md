---
status: PENDING
agent: Implementer
priority: P1
---

# Title: Add Fine-Grained Agent API Error Rate Breakdown to Telemetry and Dashboards

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) lacks fine-grained visibility into agent API errors. As detailed in the `OBSERVABILITY_AUDIT_REPORT.md`, the "Hybrid Telemetry Review" dashboard (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) lacks specific panels for fine-grained agent API error rate breakdowns. While `ohc_agent_api_errors_total` exists and is partially visualized, we need a more detailed breakdown to effectively hunt bottlenecks and self-correct the swarm across Cloud-Native and Standalone modes.

## Research Report
- **Current State:** `srcs/server/telemetry/telemetry.go` instruments `ohc_agent_api_errors_total`. The current Grafana dashboard `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` only displays the `Agent API Error Rate` aggregated by `role` and `api`.
- **Gap Analysis:** We lack fine-grained breakdowns. Specifically, we need to add `deployment_mode` (Cloud vs Standalone) and `error_code` labels to the `ohc_agent_api_errors_total` metric to differentiate between network timeouts, database lock contentions, and LLM provider errors, and then visualize these in a new panel.
- **Goal:** Provide detailed Grafana panels for `ohc_agent_api_errors_total` grouped by `deployment_mode` and `error_code` to give the human CEO actionable insights.

## Design Doc
1. **Update Telemetry Instrumentation:** Modify `srcs/server/telemetry/telemetry.go` (and wherever the error counter is incremented) to include `deployment_mode` and `error_code` attributes.
2. **Update Grafana Dashboards:** Modify `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include a new panel for "Fine-Grained Agent API Errors".
3. **Metrics Query:** Use a PromQL expression like `sum(rate(ohc_agent_api_errors_total[5m])) by (deployment_mode, error_code, api)` to expose the necessary detail.
4. **Aesthetic Excellence:** Ensure the new panel conforms to the OHC Premium Feel using standard Grafana timeseries configurations.

## Implementation Prompt
Hello Implementer agent! Please execute the following:
1. Open `srcs/server/telemetry/telemetry.go` and add `deployment_mode` and `error_code` attributes to the `agentApiErrorsCounter.Add` call. Ensure callers pass these new labels.
2. Open `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
3. Locate the existing "Agent API Error Rate" panel (or add a new one next to it).
4. Add a new timeseries panel titled "Fine-Grained Agent API Errors" that queries `sum(rate(ohc_agent_api_errors_total[5m])) by (deployment_mode, error_code, api)`.
5. Validate the JSON schema using `jq` or another tool.

## Priority
P1

## Estimated Scope
Small
