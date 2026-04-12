---
status: DONE
agent: Jules
---

# Exhaustive Hybrid Metric Mapping & Standalone Buffer Completion

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) mandates full-spectrum observability across both Cloud-Native and Standalone Desktop modes. Currently, many critical telemetry points (e.g., Cache Hits/Misses, SQLite Lock Contention, Task Processing Latency, and Agent Transition Latency) are only emitted to OpenTelemetry/Prometheus. While this works for Cloud mode, in `OHC_STANDALONE=true` mode, these metrics are lost because they do not invoke the `telemetry.BufferMetricFunc`. This creates a massive "Observability Gap" where local bottlenecks cannot be analyzed or optimized via the central dashboard after synchronization.

## Research Report
A surgical audit of `srcs/server/telemetry/telemetry.go` reveals that while `RecordTokenUsage` and `RecordAgentApiCall` correctly check for and invoke `BufferMetricFunc`, other high-fidelity metrics such as `RecordCacheHit`, `RecordSQLiteLockContention`, `RecordTaskProcessed`, and `RecordAgentTransitionLatency` only update the OTel counters. In Standalone mode, Prometheus is often unavailable, making the `telemetry_buffer` table in `swarm.db` the primary store for performance data. Without exhaustive mapping of these metrics to the buffer, our Swarm Intelligence Protocol cannot perform holistic self-correction.

## Design Doc
1. **Target Module**: `srcs/server/telemetry/telemetry.go`
2. **Logic Extension**: Every `Record*` function in the telemetry package must be updated to follow the pattern established in `RecordTokenUsage`.
3. **Pattern**:
   - Check if `BufferMetricFunc != nil`.
   - Construct a JSON payload containing the metric attributes and value.
   - Apply `RedactInterfacePII` to the payload.
   - Invoke `BufferMetricFunc(ctx, "metric_name", payloadString)`.
4. **Consistency**: Use the same `metricType` string that identifies the metric in the OTel/Prometheus configuration to ensure ease of aggregation on the Cloud side.

## Implementation Prompt
Hello Implementer agent! Your mission is to close the hybrid observability gap by ensuring all metrics are correctly buffered in Standalone mode.

1.  **Modify `srcs/server/telemetry/telemetry.go`**:
    Update the following functions to invoke `BufferMetricFunc` if it is not nil:
    - `RecordCacheHit(ctx, operation, cacheType)`
    - `RecordCacheMiss(ctx, operation, cacheType)`
    - `RecordApiRateLimitExceeded(ctx, endpoint)`
    - `RecordSQLiteLockContention(ctx, operation)`
    - `RecordSQLiteRetryExhausted(ctx, operation)`
    - `RecordTaskQueueLength(ctx, amount)`
    - `RecordTaskProcessed(ctx, latency)`
    - `RecordAgentTransitionLatency(ctx, transitionType, duration)`
    - `RecordSwarmTaskQueueLength(ctx, delta)`
    - `RecordSwarmTaskProcessingLatency(ctx, latencyMS)`
    - `RecordTaskEnqueued(ctx, taskID)`
    - `RecordTaskFailed(ctx, taskID, errStr)`

2.  **Payload Structure**: For each function, the payload should be a JSON object containing all parameters passed to the function. For example, for `RecordCacheHit`, the payload should be `{"operation": operation, "cache_type": cacheType}`.

3.  **Sanitization**: Ensure you call `RedactInterfacePII` on the payload map before marshaling it to JSON.

4.  **Verification**: Write or update tests in `srcs/server/telemetry/buffer_test.go` to verify that these functions correctly invoke the buffer function when it is configured.

5.  **Build**: Ensure all tests pass with `go test ./srcs/server/telemetry/...` or `bazelisk test //srcs/server/telemetry/...`.
