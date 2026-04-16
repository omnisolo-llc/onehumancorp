Parent: #4306

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: [telemetry] Re-implement KAIROS Sub-Agent Queue Delay & Task Claim Contention Telemetry

## Problem Statement
The previous attempt to add `SubAgentQueueDelayHistogram` and `TaskClaimContentionTotal` to telemetry failed due to PR conflicts. The KAIROS Orchestration engine is still blind to queue wait times of Sub-Agents and database lock contention rates. This observability gap prevents the Swarm from automatically scaling (Swarm Bursting) effectively based on queue pressure in Cloud-Native (Postgres) and Standalone (SQLite) modes.

## Research Report
An analysis of `srcs/server/telemetry/telemetry.go` reveals that `SubAgentQueueDelayHistogram` and `TaskClaimContentionTotal` are completely missing from the `InitWithMeter` initialization and global definitions. Cloud-Native deployments using `FOR UPDATE SKIP LOCKED` may experience different contention profiles than local SQLite transactions. Capturing this is critical for Mode Parity analysis.

## Design Doc
- **New Metrics**:
  - `SubAgentQueueDelayHistogram` (`metric.Float64Histogram`): Measures time from job enqueue to dequeue.
  - `TaskClaimContentionTotal` (`metric.Int64Counter`): Tracks the number of failed task claim attempts or retries due to lock contention.
- **Integration Points**:
  - Define in `srcs/server/telemetry/telemetry.go` under global `var` and initialize in `InitWithMeter`.
  - Export helper functions `RecordSubAgentQueueDelay(ctx context.Context, delay float64)` and `RecordTaskClaimContention(ctx context.Context, mode string)`.
- **Visual Excellence**: Downstream Grafana dashboards must adopt OHC premium visual tokens.

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
