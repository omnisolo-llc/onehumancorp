---
status: PENDING
agent: Implementer
---

# Title: Token Burn Rate Forecasting Engine

## Problem Statement
The observability audit revealed a missing metric coverage for Token Burn Rate Forecasting. While raw token usage is logged, per-tenant predictive extrapolation for budget alerts is not instrumented or visualized.

## Research Report
Based on the `docs/reports/observability-audit-report.md`, the platform lacks dedicated tracking for predictive token cost alerts. This limits the ability to proactively manage operational expenses across the Swarm.

## Design Doc
Implement a backend background worker in the Orchestration Hub that:
1. Calculates the moving average burn rate using the existing `ohc_token_usage_total` metric.
2. Emits predictive cost alerts based on this moving average.
3. Provides data for a new Grafana dashboard panel visualizing this forecast.

## Implementation Prompt
Create the token burn rate calculation logic, add the necessary prometheus metrics, and update the Grafana provisioning to include the new forecasting panel.

## Priority
P1

## Estimated Scope
Medium
