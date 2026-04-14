---
status: DONE
agent: Implementer
---

# Title: Implement Detailed Agent API Error Rate Breakdown Dashboard Panels

## Problem Statement
The OBSERVABILITY_AUDIT_REPORT.md identifies a visualization gap regarding Agent API Error Rates: "The 'Hybrid Telemetry Review' dashboard lacks specific panels for... fine-grained agent API error rate breakdowns." This limits the human CEO's ability to diagnose role-specific API invocation bottlenecks and failures across cloud-native deployments.

## Research Report
While `RecordAgentApiError` correctly records API errors into the OpenTelemetry counter `ohc_agent_api_errors_total` with attributes for `agent_id`, `role`, and `api`, the Grafana dashboard (`hybrid-telemetry.json`) only visualizes "Agent API Error Rate" as an aggregate or simple visualization, lacking fine-grained breakdown panels (e.g., error rate by role or by specific API endpoint).

## Design Doc
1. **Target File**: `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`
2. **Implementation**:
   - Add a new "API Errors by Role" panel using the expression `sum by(role) (rate(ohc_agent_api_errors_total[5m]))`.
   - Add a new "API Errors by Endpoint" panel using the expression `sum by(api) (rate(ohc_agent_api_errors_total[5m]))`.
   - Position these new panels in the "Agent Performance" section of the dashboard.
   - Apply appropriate styling consistent with the OHC-SIP aesthetic mandate.

## Implementation Prompt
Hello Implementer agent! Your mission is to close the Grafana visualization gap for fine-grained agent API error rate breakdowns.

1.  **Modify `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`**:
    Add two new timeseries panels to visualize `ohc_agent_api_errors_total`:
    - "API Errors by Role" panel: `sum by(role) (rate(ohc_agent_api_errors_total[5m]))`
    - "API Errors by Endpoint" panel: `sum by(api) (rate(ohc_agent_api_errors_total[5m]))`
2.  **Formatting**: Ensure panels use consistent styling (e.g., lines, smooth interpolation) to match existing panels.
3.  **Verification**: Confirm JSON validity using `jq . deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json > /dev/null`.

## Priority
P2

## Estimated Scope
Small
