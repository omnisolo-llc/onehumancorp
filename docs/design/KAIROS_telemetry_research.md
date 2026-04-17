<div markdown="1" style="backdrop-filter: blur(20px); saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [research] Re-implement KAIROS Sub-Agent Queue Delay & Task Claim Contention Telemetry

## Problem Statement
The KAIROS Orchestration engine lacks observability regarding queue wait times for Sub-Agents and database lock contention rates. `SubAgentQueueDelayHistogram` and `TaskClaimContentionTotal` are missing.

## Research Report
An analysis of `srcs/server/telemetry/telemetry.go` reveals that `SubAgentQueueDelayHistogram` and `TaskClaimContentionTotal` are completely missing from the `InitWithMeter` initialization and global definitions.

## Design Doc
- **New Metrics**:
  - `SubAgentQueueDelayHistogram` (`metric.Float64Histogram`): Measures time from job enqueue to dequeue.
  - `TaskClaimContentionTotal` (`metric.Int64Counter`): Tracks the number of failed task claim attempts or retries due to lock contention.
- **Integration Points**:
  - Define in `srcs/server/telemetry/telemetry.go` under global `var` and initialize in `InitWithMeter`.
  - Export helper functions `RecordSubAgentQueueDelay(ctx context.Context, delay float64)` and `RecordTaskClaimContention(ctx context.Context, mode string)`.

## Implementation Prompt
Hello Implementer!
1. Open `srcs/server/telemetry/telemetry.go`.
2. Add `SubAgentQueueDelayHistogram metric.Float64Histogram` and `TaskClaimContentionTotal metric.Int64Counter` to the global `var` block.
3. Initialize them inside `InitWithMeter(m mockableMeter)`.
4. Create helper functions `RecordSubAgentQueueDelay(ctx context.Context, delay float64)` and `RecordTaskClaimContention(ctx context.Context, mode string)`.
5. Run `bazel test //srcs/server/telemetry/...` to verify your additions.

## Priority
P1

## Estimated Scope
Small

</div>
