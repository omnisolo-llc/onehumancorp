# Title: Unify Task Queue Telemetry Metrics Between Standalone and Cloud Modes

## Problem Statement
The OHC codebase currently exhibits fragmented telemetry tracking for the task queue, with overlapping and inconsistent metrics defined in `srcs/server/telemetry/telemetry.go`. Specifically, there is `TaskQueueLengthGauge` (and its helper `RecordTaskQueueLength`), `swarmTaskQueueLengthGauge` (and `RecordSwarmTaskQueueLength`), and `ohc.sub_agent.queue_length` (tracked via `RecordQueueLength`). Furthermore, `RecordQueueLength` uses an ad-hoc local counter instead of a properly exported gauge, preventing consistent observability. This fragmentation impedes cross-mode bottleneck analysis between Cloud-Native and Standalone architectures, leading to unreliable Grafana dashboards and blind spots in horizontal scaling thresholds.

## Research Report
An audit of `srcs/server/telemetry/telemetry.go` revealed:
- `TaskQueueLengthGauge` tracks general task queue length.
- `swarmTaskQueueLengthGauge` tracks pending swarm tasks.
- `RecordQueueLength` attempts to track `ohc.sub_agent.queue_length` by initializing an ad-hoc OpenTelemetry gauge dynamically, but fails to use a globally initialized gauge like the others.

This disparity complicates observability gap analysis (per OHC-SIP), as K8s Cloud instances (using standard OpenTelemetry collectors) and SQLite Standalone instances (using fallback BufferMetricFunc) cannot reliably aggregate or compare task loads. Unifying these under a single `TaskQueueLengthGauge` or strictly defining their distinct purposes with matching initialized gauges is necessary for proper full-spectrum observability.

## Design Doc
1. Unify the queue length metrics to provide consistent visibility across architectures.
2. Standardize metric names to conform with `ohc_task_queue_length` or distinct, well-documented names like `ohc_sub_agent_queue_length`.
3. If distinct metrics are required, ensure all are declared at the package level and initialized inside `InitWithMeter(m mockableMeter)`.
4. Ensure compatibility with `BufferMetricFunc` for Standalone mode degradation.

## Implementation Prompt
Update `srcs/server/telemetry/telemetry.go` to resolve the disjointed task queue metrics.
1. If `RecordQueueLength` is intended to track sub-agent queues distinct from global tasks, declare a package-level variable `SubAgentQueueLengthGauge metric.Int64UpDownCounter`.
2. Initialize `SubAgentQueueLengthGauge` inside `InitWithMeter`.
3. Update `RecordQueueLength` to use `SubAgentQueueLengthGauge` instead of initializing one ad-hoc.
4. Verify all tests pass, specifically `telemetry_test.go` or `telemetry_extra_test.go`. Ensure metric names follow the `ohc_` prefix convention.

## Priority
P1

## Estimated Scope
Small
