# Title: Agent API Errors Grafana Dashboard Visualization

## Problem Statement
The OHC telemetry system currently exposes the `ohc_agent_api_errors_total` metric to track the total API errors made by or for agents. However, there is no corresponding high-fidelity Grafana dashboard to visualize this critical observability data. This lack of visualization violates the Full-Spectrum Observability core value that requires every feature exposing metrics to have a corresponding internal user-facing dashboard.

## Research Report
- Code review reveals that the `ohc_agent_api_errors_total` metric is exported by `src/server/telemetry/telemetry.go`.
- The `RecordAgentApiError(ctx context.Context, agentID, role, api string)` function tags this metric with `agent_id`, `role`, and `api`.
- We need to group these errors by these tags to understand which agents and APIs are failing.
- The `monitoring/dashboards` folder currently contains other dashboards like `kairos_dashboard.json`, but none for agent API errors.
- A premium UI representation is required to meet the Visual Excellence Mandate.

## Design Doc
We need a new Grafana dashboard JSON file (`monitoring/dashboards/agent_api_errors_dashboard.json`) featuring:
1.  **Total Errors (TimeSeries):** Visualizing `rate(ohc_agent_api_errors_total[1m])` grouped by `api` and `role`.
2.  **Errors by Agent (Bar/TimeSeries):** Visualizing the errors grouped by `agent_id` to identify specific failing agents.
3.  **Aesthetic Adherence:** Ensure the global CSS styles (glassmorphism tokens) are injected natively via Grafana theming to maintain the OHC Premium Feel, avoiding raw `<style>` injections to prevent XSS risks.

## Implementation Prompt
1. Create a new file `monitoring/dashboards/agent_api_errors_dashboard.json`.
2. Configure it as a Grafana dashboard with panels for the `ohc_agent_api_errors_total` metric.
3. Ensure panels group by `api`, `role`, and `agent_id`.
4. Apply the OHC glassmorphism tokens natively within Grafana's UI theming configurations without relying on raw HTML/CSS injections in Text panels.
5. Run tests if applicable (though this is purely a dashboard JSON creation task).
6. Submit a PR.

## Priority
P1

## Estimated Scope
Small
