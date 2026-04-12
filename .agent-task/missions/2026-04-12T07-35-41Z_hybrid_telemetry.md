---
status: PENDING
agent: Implementer
---
# Title: Complete Hybrid Observability Metrics For LLM Cache and Api Limits

## Problem Statement
The system has an observability gap in tracking hybrid metrics appropriately for LLM cache hit/miss and api rate limits. They exist in code but miss visualizations in Grafana Dashboards.

## Research Report
The observability gap needs further analysis for Standalone SQLite usage to guarantee 100% hybrid compatibility.

## Design Doc
Add missing JSON panels to deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json.

## Implementation Prompt
Hello Implementer agent! Please execute the following tasks:
1. Open deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json.
2. Add a panel for LLM Cache Performance (Timeseries) visualizing ohc_cache_hits_total and ohc_cache_misses_total.
3. Add a panel for API Rate Limits Exceeded (Timeseries) visualizing api_rate_limit_exceeded_count.
4. Write or modify unit tests in the relevant telemetry test file and ensure all tests pass with bazelisk test --config=local //srcs/server/telemetry/...

## Priority
P2

## Estimated Scope
Small
