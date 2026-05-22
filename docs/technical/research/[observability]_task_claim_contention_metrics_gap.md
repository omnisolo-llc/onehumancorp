Parent: #EpicID

# Title: Add Dashboard Visualization for Task Claim Contention

## Problem Statement
The `ohc_task_claim_contention_total` metric is emitted by the backend to track failed task claim attempts due to database lock contention. However, it is not currently visualized in our Grafana dashboards (`kairos_hybrid_metrics.json`), leaving a critical observability gap for diagnosing bottleneck conditions during Sub-Agent queue scaling in Cloud vs Standalone modes.

## Research Report
Audit shows the metric exists in `src/server/telemetry/telemetry.go` but no dashboard panels currently query it. This prevents tracking whether task claims fail due to PostgreSQL row locks or SQLite database locks.

## Design Doc
Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to include a new panel visualizing the rate of `ohc_task_claim_contention_total` grouped by `mode`.

## Implementation Prompt
Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` with a new panel querying `sum(rate(ohc_task_claim_contention_total[5m])) by (mode)`.

## Priority
P2

## Estimated Scope
Small
