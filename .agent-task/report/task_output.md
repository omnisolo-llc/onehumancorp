# [research] Telemetry Buffer Inefficiency Issue Brief

## Title
Resolve Synchronous Telemetry Bottleneck in Standalone Mode

## Problem Statement
In Standalone Mode, telemetry metrics are buffered to SQLite synchronously inside `BufferMetricFunc`. This synchronous execution flow within the critical path of application operations introduces significant performance bottlenecks, particularly during high-frequency telemetry logging. For a non-technical business owner, this latency manifests as an unresponsive or sluggish mobile/desktop application during heavy usage, severely degrading the user experience.

## Research Report
- **Analysis:**
  - `telemetry.go` defines `InitStandaloneBuffer` which configures `BufferMetricFunc`.
  - Currently, `BufferMetricFunc` performs synchronous JSON unmarshaling, PII redaction (`RedactInterfacePII`), JSON marshaling, and a blocking SQLite `ExecContext` call for every single telemetry event.
  - This synchronous processing blocks the calling thread, increasing latency for core operations.
- **Insights:**
  - High-frequency metrics must be offloaded from the synchronous execution path.
  - An asynchronous buffer mechanism (e.g., Go channels and a background worker goroutine) is required to handle metric insertion without blocking the main execution flow.
- **Recommendations:**
  - Refactor `InitStandaloneBuffer` to utilize an asynchronous processing queue.
  - The calling function should rapidly push events to a channel and return immediately.
  - A background worker should read from the channel, perform PII redaction, and execute the SQLite insertion.

## Design Doc
1. **Component Update:** Modify `InitStandaloneBuffer` in `src/server/telemetry/sync_daemon.go`.
2. **Architecture:**
   - Introduce an event channel (`chan bufferedMetric`) to hold incoming telemetry data.
   - Spawn a background worker goroutine during `InitStandaloneBuffer` initialization.
   - The worker will continuously listen on the channel, process the payload (JSON unmarshal, PII redact, JSON marshal), and insert it into the `telemetry_buffer` SQLite table.
3. **Data Flow:**
   - Caller -> `BufferMetricFunc` -> Event Channel -> Background Worker -> SQLite.

## Implementation Prompt
"Refactor `InitStandaloneBuffer` in `src/server/telemetry/sync_daemon.go` to process telemetry metrics asynchronously. Introduce a Go channel and a background worker goroutine to handle JSON processing, PII redaction, and SQLite insertions. Ensure that `BufferMetricFunc` pushes to the channel without blocking, preventing performance regressions in Standalone Mode."

## Priority
P1

## Estimated Scope
Medium
