---
status: PENDING
agent: Implementer
priority: P1
---

# Title: Integrate Advanced Telemetry for Standalone to Cloud Synchronization Daemon

## Problem Statement
The OHC Hybrid Architecture relies heavily on the `autodream_sync` daemon to synchronize state between the local Standalone instance (SQLite) and the Cloud backend (Postgres/Redis) for premium observability and continuity. Currently, the sync process lacks deep instrumentation via OpenTelemetry to trace sync latency, record failure counts, and expose these metrics to Grafana.

## Research Report
- **Market Context**: Competing offline-first syncing protocols provide detailed telemetry on sync batch sizes, error rates, and delta application times. OHC currently only does basic logging.
- **OHC Requirement**: The `srcs/server/sync/autodream_sync.go` daemon needs integration with OpenTelemetry (`go.opentelemetry.io/otel/metric` and `go.opentelemetry.io/otel/trace`).
- **Tooling Discovery**: We need to instrument `SyncLoop` and `syncBatch` functions in `autodream_sync.go` to emit metrics: `sync_latency_ms`, `sync_error_count`, `sync_batch_size`.

## Design Doc
- **Module Path**: `srcs/server/sync`
- **Architecture**:
  - Add `meter` and `tracer` initialization to `NewAutoDreamSyncDaemon`.
  - Instrument `syncBatch` to measure duration.
  - Increment `sync_error_count` on failure.
  - Record the number of items synced in `sync_batch_size`.
  - Ensure the metrics are exported following OHC's observability heartbeat rules.

## Implementation Prompt
Hello Implementer agent!
1. Modify `srcs/server/sync/autodream_sync.go` to import `go.opentelemetry.io/otel`, `go.opentelemetry.io/otel/metric`, and `go.opentelemetry.io/otel/trace`.
2. Add telemetry instrumentation to track `sync_latency_ms`, `sync_error_count`, and `sync_batch_size`.
3. Update `srcs/server/sync/BUILD.bazel` to include telemetry dependencies if missing.
4. Ensure the `autodream_sync_test.go` still passes and add test cases to cover the new telemetry logic.
5. Create a status file in `.agent-task/status/` marking completion.

## Priority
P1

## Estimated Scope
Medium
