---
status: DONE
agent: Nova
priority: P1
---

# Title: Implement Hybrid Observability Token Burn Rate & Local Metrics Sync

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) previously exhibited observability gaps and mode-specific bottlenecks.
However, a recent audit confirms these features are already implemented in the codebase:
1. **Cloud-Native Mode:** `TokenForecastWorker` tracks and forecasts per-tenant Token Burn Rates (`ohc_token_burn_rate_forecast`).
2. **Standalone Desktop Mode:** Local SQLite (`swarm.db`) concurrency is throttled via `OHC_MULTITENANT=false`. Local telemetry is buffered and synced to the Cloud DB via `SyncBufferedMetrics`.

## Research Report
A comprehensive audit of `OBSERVABILITY_AUDIT_REPORT.md` and `TELEMETRY_REPORT.md` highlights the divergent bottlenecks between Cloud-Native and Standalone modes:
- **Cloud-Native:** The primary inefficiencies arise from network latency to external LLMs and PostgreSQL lock contention during massive `agent_missions` bulk updates. Error rates spike during aggressive pod scaling.
- **Standalone:** Single-user local Go backend operations are constrained by host I/O limits, rapidly hitting SQLite lock contention as exponential backoff is exhausted during parallel operations.

## Design Doc
A review of the codebase confirms:
1. **Token Burn Rate Forecasting Engine:** Implemented via `TokenForecastWorker` in `telemetry.go` and `StartTokenBurnForecaster` in `hub.go`.
2. **Standalone Concurrency Throttling:** Implemented in `sip.go` via `acquireThrottle` when `OHC_MULTITENANT` is false.
3. **Standalone Local Metric Buffer:** Implemented via `BufferMetric` in `sip.go` and `StartSyncDaemon` in `main.go`.

## Implementation Prompt
The features are already present in the codebase. Verify their existence and mark this mission as DONE without modifying the code to generate a fake diff.

## Priority
P1

## Estimated Scope
Medium

## Audit Results
A recent audit confirms that the requested features have already been fully implemented in the codebase:
1. **Token Burn Rate Forecasting Engine:** Implemented via `TokenForecastWorker` in `telemetry.go` and `StartTokenBurnForecaster` in `hub.go`. The Grafana dashboard was also successfully verified.
2. **Standalone Concurrency Throttling:** Implemented in `sip.go` via `acquireThrottle` when `OHC_MULTITENANT` is false.
3. **Standalone Local Metric Buffer:** Implemented via `BufferMetric` in `sip.go` and `StartSyncDaemon` in `main.go`.

As the feature requirements are met, this mission is being marked as DONE to close out the ticket without introducing redundant or unnecessary code changes.
