---
status: DONE
agent: Miser
---

# Title: Expose Token Cost Metrics for LLM Usage

## Problem Statement
While OHC-HA currently supports LLM usage (e.g., MinimaxClient) and token compression features, there's no native observability for real-time tracking of token usage costs across different LLM interactions in the Hybrid architecture. As the principal Cost Engineer (Miser), exposing high-fidelity Prometheus metrics for LLM token burn is critical to achieving the OHC Full-Spectrum Observability mandate.

## Research Report
- Current metrics exist for telemetry but lack explicit token tracking.
- The `MinimaxClient` methods (`Reason`, `GenerateEmbedding`) could easily increment a prometheus counter for tokens consumed, parsed from the responses if available or estimated.
- Exposing these metrics allows Grafana dashboards to track LLM operational costs over time.

## Design Doc
1. **Telemetry Update**:
   - Add new OpenTelemetry counters in `srcs/server/telemetry/telemetry.go` for `llm_tokens_total` (labeled by `model`, `type` e.g., prompt vs completion, and `operation`).
2. **MinimaxClient Integration**:
   - Update `srcs/server/orchestration/minimax.go` (or `cached_minimax.go` if it wraps it) to observe token usage on each successful call.
3. **Dashboards**:
   - Create a premium UI dashboard component in `srcs/app/lib/dashboard.dart` or related to visualize token burn using Glassmorphism. Or at least verify the metric is exposed. Let's stick to the server-side metrics first.

## Implementation Prompt
- Add the `llm_tokens_total` metric.
- Increment it in the LLM client.
- Ensure all tests pass.

## Priority
P1

## Estimated Scope
Small
