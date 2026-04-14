---
title: "Fix Sub-Agent Queue Metric Telemetry Registration"
status: FAILED
agent: Implementer
priority: P1
scope: Small
---

# Fix Sub-Agent Queue Metric Telemetry Registration

## Problem Statement
The OHC codebase tracks `sub_agent_queue_length` through `RecordQueueLength` in `srcs/server/telemetry/telemetry.go`. However, this metric is currently dynamically instantiated via `meter.Int64UpDownCounter` on every call, and is not properly registered globally like other metrics. This causes inefficiencies and observability gaps.

## Research Report
The function `RecordQueueLength` currently creates an UpDownCounter dynamically every time it is called:
```go
	gauge, err := meter.Int64UpDownCounter(
		"ohc.sub_agent.queue_length",
		metric.WithDescription("The current number of jobs in the sub-agent task queue"),
	)
```
Memory rules state: "When initializing OpenTelemetry metrics in Go, declare and instantiate them directly as global variables within a `var` block (e.g., `var MetricName, _ = meter.Int64Counter(...)`) rather than inside an `init()` function."
Note that the `telemetry.go` module registers metrics in `InitWithMeter(m mockableMeter) error`. So the global variable should be defined globally and assigned within `InitWithMeter`.

## Design Doc
1. Define a global variable `subAgentQueueLengthGauge metric.Int64UpDownCounter` in `srcs/server/telemetry/telemetry.go` near other gauge declarations.
2. Inside `InitWithMeter(m mockableMeter) error`, initialize `subAgentQueueLengthGauge`:
   ```go
   subAgentQueueLengthGauge, err = m.Int64UpDownCounter(
       "ohc.sub_agent.queue_length",
       metric.WithDescription("The current number of jobs in the sub-agent task queue"),
   )
   if err != nil {
       errs = append(errs, err)
   }
   ```
3. Update `RecordQueueLength` to use the global variable:
   ```go
   func RecordQueueLength(ctx context.Context, delta int) {
       if BufferMetricFunc != nil {
           BufferMetricFunc(ctx, "sub_agent_queue_length", fmt.Sprintf("%d", delta))
           return
       }
       if subAgentQueueLengthGauge != nil {
           subAgentQueueLengthGauge.Add(ctx, int64(delta))
       }
   }
   ```
4. Verify tests pass.

## Implementation Prompt
Implement the changes described in the Design Doc in `srcs/server/telemetry/telemetry.go`. Do not use `panic` on error.
