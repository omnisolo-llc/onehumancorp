Parent: #EpicID

# Title: Add Mode-Specific Prometheus Metrics to Distributed Lock Mutexes

## Problem Statement
The `MutexProvider` in `srcs/server/orchestration/mutex.go` implements distributed locks for both Cloud (Redis) and Standalone (SQLite) modes. Currently, there are no metrics emitted when a lock is successfully acquired, failed to acquire, or unlocked. This creates an observability blindspot, making it difficult to debug lock contention, latency, and deadlock issues in multi-tenant environments versus local setups.

## Research Report
An audit of `srcs/server/orchestration/mutex.go` shows that `RedisMutex` and `SQLiteMutex` handle lock operations silently. While `srcs/server/telemetry/telemetry.go` tracks lock contention for databases, it does not explicitly track the generic distributed mutex acquisition logic. Grafana dashboards (like `kairos_hybrid_metrics.json`) do not visualize mutex lock rates or durations, which is critical for the OHC Full-Spectrum Observability requirement.

## Design Doc
1. Define Prometheus metrics in `srcs/server/telemetry` for mutex operations: `MutexAcquisitionTotal` (Counter), `MutexReleaseTotal` (Counter), `MutexContentionTotal` (Counter), and `MutexAcquisitionDuration` (Histogram). Include `mode` (redis/sqlite) and `key` as labels.
2. Update `srcs/server/orchestration/mutex.go` to inject the telemetry context and record metrics upon calling `Lock` and `Unlock`. Record time taken to acquire lock if necessary.
3. Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to include panels for Mutex Contention and Acquisition Duration by Mode. Apply OHC Premium Glassmorphism styling natively.

## Implementation Prompt
You are an Implementer. Implement the design above:
1. Create functions in `srcs/server/telemetry/telemetry.go` to record mutex operations with `mode` and `key` labels.
2. Instrument `RedisMutex.Lock`, `RedisMutex.Unlock`, `SQLiteMutex.Lock`, and `SQLiteMutex.Unlock` in `srcs/server/orchestration/mutex.go` to call these telemetry functions.
3. Add panels to `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to visualize these new metrics.
4. Ensure tests pass by running `bazelisk test //...`.

## Priority
P1

## Estimated Scope
Medium
