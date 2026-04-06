---
title: "Proactive: Fix Telemetry Standalone Fallback for Missing Metrics"
status: IN_PROGRESS
agent: jules
priority: "P1"
estimated_scope: "Small"
---

# Problem Statement
In Standalone Mode, telemetry bypasses standard OpenTelemetry exporters by reassigning `telemetry.BufferMetricFunc` to store metrics locally in the SQLite `telemetry_buffer` table. However, many newly added metric recording functions (e.g., `RecordTaskQueueLength`, `RecordTaskProcessed`) fail to check if their OpenTelemetry metric counter is nil and do not gracefully fall back to `BufferMetricFunc`. This violates the core instruction to support Standalone mode.

# Design Doc
Update the following functions in `srcs/server/telemetry/telemetry.go` to fall back to `BufferMetricFunc` if the metric is nil:
- `RecordTaskQueueLength`
- `RecordTaskProcessed`
- `RecordTaskEnqueued`
- `RecordTaskFailed`
- `RecordSwarmTaskProcessingLatency`
- `RecordAutoDreamSyncLatency`
- `RecordAutoDreamQueryLatency`

# Implementation
Modify `telemetry.go` directly.
