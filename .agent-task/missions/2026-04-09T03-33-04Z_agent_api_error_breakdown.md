---
status: PENDING
agent: Researcher
---
# Title: Agent API Error Rate Breakdown Visualization

## Problem Statement
The Observability Audit Report identified a Grafana Visualization Gap: the "Hybrid Telemetry Review" dashboard lacks specific panels for token forecasting and fine-grained agent API error rate breakdowns.

## Research Report
The metrics `ohc_token_burn_rate_forecast` and `ohc_agent_api_errors_total` exist in the backend telemetry (`srcs/server/telemetry/telemetry.go`). However, the existing `hybrid-telemetry.json` and `token-forecast.json` dashboards lack comprehensive visual panels to effectively break down these errors by agent role and track the forecasted token burn rates accurately to prevent budget overruns in both Cloud and Standalone architectures.

## Design Doc
Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`:
- Add a new "Time series" panel displaying the `ohc_agent_api_errors_total` metric broken down by `role` and `api`.
- Ensure the visual design aligns with the Glassmorphism OHC Stylistic Intent Profile (OHC-SIP).
- Add a "Gauge" panel for `ohc_token_burn_rate_forecast` to visualize the moving average token burn rate per minute.

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. Update the `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include two new panels.
2. The first panel should visualize `sum(rate(ohc_agent_api_errors_total[5m])) by (role, api)`.
3. The second panel should visualize `ohc_token_burn_rate_forecast` with alerting thresholds.
4. Ensure the dashboard modifications do not break the existing JSON schema and maintain the premium visual style.

## Priority
P1

## Estimated Scope
Small
