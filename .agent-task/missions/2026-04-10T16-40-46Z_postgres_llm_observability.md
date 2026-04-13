---
status: PENDING
agent: Implementer
---
# Title: Proactive Hybrid Telemetry: Postgres Lock Contention and LLM Network Latency

## Problem Statement
The OBSERVABILITY_AUDIT_REPORT.md identifies "Network Latency to external LLM providers and PostgreSQL Lock Contention during massive agent_missions bulk updates" as key bottlenecks in Cloud-Native mode. However, the existing telemetry.go package only tracks SQLite metrics and completely lacks specific tracking for Postgres lock contention and external LLM latency per model.

## Research Report
Auditing srcs/server/telemetry/telemetry.go and deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json confirms no panels or metrics exist for ohc_postgres_lock_contention_total or ohc_llm_network_latency_seconds. Without these metrics, the Swarm Intelligence Protocol cannot correctly auto-scale pods or degrade gracefully under load in Cloud-Native mode.

## Design Doc
- **Go Telemetry Integration**: Add OpenTelemetry counters/histograms for ohc_postgres_lock_contention_total and ohc_llm_network_latency_seconds in srcs/server/telemetry/telemetry.go.
- **Orchestration Hook**: Locate the Postgres lock handlers in srcs/server/orchestration/sip.go (such as transaction code using FOR UPDATE SKIP LOCKED) and invoke the postgres lock telemetry when it encounters contention or specific timeout errors.
- **Grafana Dashboard**: Add corresponding visual panels in deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json styled with OHC Glassmorphism CSS tokens.

## Implementation Prompt
Hello Implementer agent! Please execute the following tasks:
1. **Locate Code**: Grep the codebase for FOR UPDATE SKIP LOCKED and the srcs/server/telemetry/telemetry.go file to find where to add these hooks. Also locate the telemetry test file (e.g., telemetry_test.go) for writing tests.
2. **Telemetry**: Open srcs/server/telemetry/telemetry.go and declare new metrics postgresLockContentionCounter metric.Int64Counter and llmNetworkLatencyHistogram metric.Float64Histogram.
3. **Metric Functions**: Implement RecordPostgresLockContention(ctx context.Context, operation string) and RecordLLMNetworkLatency(ctx context.Context, model string, latency float64). Ensure graceful error handling instead of panics during initialization.
4. **SIP Hook**: Open srcs/server/orchestration/sip.go, find the code handling PostgreSQL FOR UPDATE SKIP LOCKED, and inject a call to telemetry.RecordPostgresLockContention(ctx, "upsert_mission") if the query returns an error indicating no rows or a lock timeout.
5. **Grafana Dashboards**: Open deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json and append two new JSON panels for "Postgres Lock Contention Rate" and "LLM Network Latency".
6. **Verification**: Write or modify unit tests in the relevant telemetry test file and ensure all tests pass with bazelisk test --config=local //...

## Priority
P1

## Estimated Scope
Medium
