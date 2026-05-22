# [observability] LLM Provider Latency Dashboard

## Title
Create LLM Provider Latency & Error Rate Dashboard

## Problem Statement
While OHC tracks token usage efficiently across providers (e.g., Minimax, GPT-4), we lack granular observability into API response latencies and provider-specific error rates in our Grafana dashboards. The absence of this visualization prevents operators from detecting sudden API degradation, timeout bottlenecks, or rate-limiting events from specific LLM providers.

## Research Report
- **Context**: The `src/server/telemetry/minimax_metrics.rs` and `src/server/telemetry/telemetry/mod.rs` track token usage, but there is no dedicated dashboard for provider health.
- **Competitor Analysis**: Leading AI platforms provide real-time dashboards mapping provider latency (P50, P90, P99) and error rates (429, 500, etc.) to quickly swap models if one degrades.
- **Gap**: The `monitoring/dashboards` directory lacks a dedicated LLM Provider Health dashboard. We are currently blind to external provider performance spikes.

## Design Doc
1. **File Location**: Create `monitoring/dashboards/llm_provider_health.json`.
2. **Dashboard Structure**:
   - **Row 1: Overview**: Total requests by provider, Global Error Rate.
   - **Row 2: Latency**: P50, P90, P99 API response latencies per provider.
   - **Row 3: Error Rate**: HTTP 4xx and 5xx error rates by provider.
3. **Visual Excellence**: All panels must use OHC premium CSS tokens. For text panels, inject global `<style>` blocks to maintain the Glassmorphism visual identity.
4. **Data Source**: Prometheus.

## Implementation Prompt
Hello Implementer agent! Your task is to resolve the observability gap for LLM Provider Health.

1. Create a new file `monitoring/dashboards/llm_provider_health.json`.
2. Configure it as a Grafana dashboard with panels to visualize LLM provider latency and error rates.
3. Ensure the panels group by provider (e.g., `model` or `provider` label) to visualize differences in performance.
4. Add a "Text" panel that injects the required CSS global styles for the visual excellence mandate (`<style> * { font-family: 'Outfit', 'Inter', sans-serif; } .panel-container { backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03) !important; border: 1px solid rgba(255, 255, 255, 0.1) !important; } </style>`).
5. Run tests if applicable (though this is purely a dashboard JSON creation task).
6. Submit a PR.

## Priority
P1

## Estimated Scope
Small
