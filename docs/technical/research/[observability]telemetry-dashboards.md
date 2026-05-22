<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Issue Brief: KAIROS Metrics Grafana Dashboard Visualization

## Problem Statement
The OHC `kairos` module in `src/server/orchestration/kairos/metrics.rs` effectively captures distributed state machine transitions, transition durations, and task queue depths via Prometheus metrics (`TransitionsTotal`, `TransitionDuration`, `TaskQueueDepth`). However, there is no corresponding high-fidelity Grafana dashboard to visualize this critical observability data for the hybrid architecture (cloud vs. standalone mode). This gap violates the Full-Spectrum Observability core value that requires every feature exposing metrics to have a corresponding internal user-facing dashboard.

## Research Report
- Code review reveals that `ohc_kairos_transitions_total`, `ohc_kairos_transition_duration_seconds`, and `ohc_agent_task_queue_depth` are exported by `src/server/orchestration/kairos/metrics.rs`.
- The `GetMode()` function tags these metrics with `mode` (cloud, standalone, headless).
- The `TransitionsTotal` metric is additionally tagged with `status` (destination state).
- The `monitoring/dashboards` folder currently only contains `chaos_dashboard.json`.
- Competitors provide out-of-the-box UI for agent queue observability. OHC needs a premium UI representation.

## Design Doc
We need a new Grafana dashboard JSON file (`monitoring/dashboards/kairos_dashboard.json`) featuring:
1.  **Task Queue Depth (Gauge/TimeSeries):** Visualizing `ohc_agent_task_queue_depth` grouped by `mode`.
2.  **State Machine Transitions Rate (TimeSeries):** Visualizing `rate(ohc_kairos_transitions_total[1m])` grouped by `mode` and `status`.
3.  **Transition Duration (Heatmap/TimeSeries):** Visualizing `rate(ohc_kairos_transition_duration_seconds_sum[1m]) / rate(ohc_kairos_transition_duration_seconds_count[1m])` grouped by `mode`.
4.  **Aesthetic Adherence:** Ensure the global CSS styles (glassmorphism tokens) are injected into a Text panel on the dashboard to maintain the OHC Premium Feel.

## Implementation Prompt
1. Create a new file `monitoring/dashboards/kairos_dashboard.json`.
2. Configure it as a Grafana dashboard with panels for the `ohc_kairos_transitions_total`, `ohc_kairos_transition_duration_seconds`, and `ohc_agent_task_queue_depth` metrics.
3. Ensure panels group by `mode` to visualize Cloud vs. Standalone differences.
4. Add a "Text" panel that injects the required CSS global styles for the visual excellence mandate (`<style> * { font-family: 'Outfit', 'Inter', sans-serif; } .panel-container { backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03) !important; border: 1px solid rgba(255, 255, 255, 0.1) !important; } </style>`).
5. Run tests if applicable (though this is purely a dashboard JSON creation task).
6. Submit a PR.

## Priority
P1

## Estimated Scope
Small
</div>
