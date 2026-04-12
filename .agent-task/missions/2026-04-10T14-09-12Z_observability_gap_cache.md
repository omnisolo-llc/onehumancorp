---
status: DONE
agent: Jules
priority: P1
---

# Title: Observability Gap: Implement LLM Cache & Rate Limit Grafana Panels

## Problem Statement
While the backend Go code actively records critical metrics regarding LLM cache performance (`ohc_cache_hits_total`, `ohc_cache_misses_total`) and API rate limiting (`api_rate_limit_exceeded_count`), these metrics are completely absent from the Grafana provisioning configurations (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and others). This creates an observability gap where operators cannot monitor the effectiveness of caching strategies or detect API rate limit throttling during burst executions.

## Research Report
An audit of `srcs/server/telemetry/telemetry.go` alongside Grafana dashboards reveals:
- **Metrics Available**:
  - `ohc_cache_hits_total` (with `operation` and `cache_type` labels)
  - `ohc_cache_misses_total` (with `operation` and `cache_type` labels)
  - `api_rate_limit_exceeded_count` (with `endpoint` label)
- **Dashboards Missing Panels**:
  - The `hybrid-telemetry.json` dashboard lacks any visualization for cache hits/misses and API rate limit occurrences.
- **Impact**: Without these panels, the operational overhead LLM inference latency cannot be accurately attributed to cache misses vs network latency, and throttling from LLM providers (HTTP 429) is silent in the primary dashboard.

## Design Doc
1. **Grafana Dashboards Update**:
   - Target File: `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`
   - Add a new "LLM Cache Performance" panel that displays cache hits and misses.
     - Expression 1: `sum(rate(ohc_cache_hits_total[5m])) by (cache_type)`
     - Expression 2: `sum(rate(ohc_cache_misses_total[5m])) by (cache_type)`
   - Add a new "API Rate Limits Exceeded" panel.
     - Expression: `sum(rate(api_rate_limit_exceeded_count[5m])) by (endpoint)`
   - Ensure the panels are correctly positioned and follow the JSON structure of existing timeseries panels in the dashboard.

## Implementation Prompt
Hello Implementer agent! Please address the observability gap by adding the missing metric visualizations.
1. Modify `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. Add a panel for **LLM Cache Performance** (Timeseries) visualizing `ohc_cache_hits_total` and `ohc_cache_misses_total`.
3. Add a panel for **API Rate Limits Exceeded** (Timeseries) visualizing `api_rate_limit_exceeded_count`.
4. Ensure the dashboard `$datasource` variable is used correctly.
5. Use `bazelisk test //...` (or specifically target relevant telemetry/infra test targets) to ensure no embed JSON validation tests break.

## Priority
P1

## Estimated Scope
Medium
