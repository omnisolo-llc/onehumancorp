---
status: DONE
agent: Nova
priority: P1
scope: Small
---

# Title: Proactive Implementer Growth Improvements: Add Observability to Analytics and Growth Experiments

## Problem Statement
The OHC Hybrid Architecture dictates that every feature MUST expose high-fidelity metrics via OpenTelemetry and Prometheus. Currently, the local analytics tracker, growth experiments manager, and referrals lack proper Prometheus counter tracking. This prevents internal observability of experiment traffic split, feature conversion rates, and event tracking metrics.

## Research Report
- OHC-SIP Core Values dictate "Full-Spectrum Observability".
- `services/growth/experiments.go` needs to track which experiment variants are evaluated.
- `services/growth/referrals.go` needs to track total successful referrals.
- `lib/analytics/tracker.go` needs to emit a metric count for every successful event tracked.
- Since no explicit missions were pending for my domain, I am completing this proactive task to maintain the Gold Standard.

## Design Doc
1. Implement Prometheus `prometheus.NewCounterVec` named `ohc_growth_experiments_total` in `services/growth/experiments.go` to track variant assignments.
2. Implement Prometheus `prometheus.NewCounter` named `ohc_growth_referrals_total` in `services/growth/referrals.go`.
3. Implement Prometheus `prometheus.NewCounterVec` named `ohc_analytics_events_total` in `lib/analytics/tracker.go`.
4. Update `BUILD.bazel` files with `@com_github_prometheus_client_golang//prometheus` dependency.

## Implementation Prompt
1. Add `ohc_growth_experiments_total` to `services/growth/experiments.go`.
2. Add `ohc_growth_referrals_total` to `services/growth/referrals.go`.
3. Add `ohc_analytics_events_total` to `lib/analytics/tracker.go`.
4. Ensure tests pass via `bazelisk test //...`.

## Priority
P1

## Estimated Scope
Small
