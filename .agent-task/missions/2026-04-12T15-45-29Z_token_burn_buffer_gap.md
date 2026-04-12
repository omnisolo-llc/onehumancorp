---
status: IN_PROGRESS
agent: Jules
---

# Title: Observability Gap: Missing Local Metric Buffer for Token Burn Rate

## Problem Statement
In Standalone Desktop mode, local metrics must be buffered and synced with the Cloud DB using the localized metric buffer feature in `srcs/server/telemetry/telemetry.go`. However, `RecordTokenBurnRate` in `srcs/server/telemetry/telemetry.go` is missing the call to `BufferMetricFunc`, meaning token burn rate metrics are not buffered or synchronized when running in Standalone mode.

## Research Report
An audit of `srcs/server/telemetry/telemetry.go` reveals that while other metrics (e.g., `token_usage`, `agent_api_call`) implement `BufferMetricFunc`, `RecordTokenBurnRate` does not. Because Prometheus cannot scrape local SQLite metrics efficiently, the local buffer is critical for capturing this telemetry when offline.

## Design Doc
1. **Telemetry Interceptor Update**: Modify `RecordTokenBurnRate` in `srcs/server/telemetry/telemetry.go` to include support for buffering when `BufferMetricFunc` is set.
If `BufferMetricFunc != nil`, call `BufferMetricFunc(ctx, "token_burn_rate_forecast", string(payloadBytes))` with a JSON payload containing `organization_id` and `rate`.
2. **Test Coverage**: Add a call to `RecordTokenBurnRate(ctx, "org1", 5.5)` within `TestBufferMetricFunc` in `srcs/server/telemetry/buffer_test.go` to ensure the buffer function is correctly invoked and test coverage remains high.

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. In `srcs/server/telemetry/telemetry.go`, update `RecordTokenBurnRate` to check for and invoke `BufferMetricFunc` with the event `token_burn_rate_forecast`, ensuring PII redaction if necessary.
2. In `srcs/server/telemetry/buffer_test.go`, add a test case in `TestBufferMetricFunc` to verify that `RecordTokenBurnRate` correctly invokes the buffer.
3. Ensure appropriate OpenTelemetry wrappers are maintained.
4. Write and run tests, verifying them with `~/go/bin/bazelisk test //srcs/server/telemetry/...`.

## Priority
P2

## Estimated Scope
Small
