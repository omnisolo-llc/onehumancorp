---
status: DONE
agent: jules
---
# Title: Fix Grafana Dashboard Prometheus Histogram Queries

## Problem Statement
The Grafana dashboards for the Hybrid Agentic OS are currently querying the raw `_bucket` metric for `http_request_duration_seconds_bucket` directly within `histogram_quantile` without properly aggregating by the `le` label using `sum`. This causes invalid visualizations and incorrect 95th percentile latency calculations across both Cloud-native (multi-tenant) and Standalone (local) contexts.

## Research Report
While performing an Observability Gap Analysis across the Hybrid Telemetry, I examined the Prometheus queries in the Grafana dashboards.
Specifically, `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` and `deploy/helm/ohc/dashboards/hybrid-telemetry.json` contain the following expression:
`"expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"`

According to Prometheus documentation and project standards, calculating a quantile over a histogram requires aggregating the rates across all dimensions except `le` (less than or equal to) using `sum`. Failing to sum the rates leads to vector matching errors or nonsensical quantile values when there are multiple series (e.g., multiple instances, methods, or paths).

## Design Doc
To resolve this, we need to update the PromQL expressions in the affected Grafana dashboard files.
The incorrect expression:
`histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))`
Should be replaced with:
`histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))`

This change must be applied to both the Docker and Helm deployment configurations to ensure the fix is propagated across all environments.

## Implementation Prompt
Hello Implementer,

Please fix the Prometheus histogram queries in the Grafana dashboards.

1. Edit the file `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`. Search for `"expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"` and replace it with `"expr": "histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))"`.
2. Edit the file `deploy/helm/ohc/dashboards/hybrid-telemetry.json`. Search for `"expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"` and replace it with `"expr": "histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))"`.
3. Verify the JSON syntax is still valid after your edits.

Acceptance Criteria:
- The `http_request_duration_seconds_bucket` query in both files correctly uses `sum(...) by (le)`.
- No other dashboard queries are unintentionally modified.

## Priority
P1

## Estimated Scope
Small
