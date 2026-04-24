# [Observability] Standardize Deployment Mode Tags on Swarm Telemetry

## Title
Standardize Deployment Mode Tags on Swarm Telemetry

## Problem Statement
Swarm operators are unable to accurately distinguish between task queue contention in Cloud-native (multi-tenant) vs Standalone (local) contexts. Although some metrics (like harness execution latencies) are correctly tagged with `deployment_mode`, critical Swarm health metrics like `ohc_swarm_task_queue_length` and `ohc_task_queue_length` lack this crucial dimension. This observability gap prevents Swarm self-correction mechanisms from identifying environment-specific bottlenecks. From the perspective of a swarm operator, it's impossible to tell if a queue buildup is happening across all users on the main cloud infrastructure or just for isolated users running the platform locally, which delays critical interventions.

## Research Report
**Hybrid Telemetry Review & Observability Gap Analysis:**
A thorough audit of `src/server/telemetry/telemetry.go` and Grafana dashboards reveals an inconsistency in metric tagging.
- Metrics such as `HarnessInitLatency`, `HarnessDbIoLatency`, `HarnessExecutionLatency`, `autoDreamSyncDuration`, and `meshBroadcastTotal` successfully utilize the `deployment_mode` attribute to differentiate between Cloud and Standalone environments.
- Conversely, queue-focused metrics, notably `swarmTaskQueueLengthGauge`, `TaskQueueLengthGauge`, and `subAgentQueueLengthGauge`, do not record the `deployment_mode`.

**Bottleneck Hunting & Swarm Health Assessment:**
The lack of a `deployment_mode` tag on queue metrics causes Swarm health dashboards to aggregate local standalone metrics with global cloud multi-tenant metrics. This severely obscures cost efficiency analysis and bottleneck hunting. For example, if a localized network partition in a standalone instance leads to a sudden spike in `ohc_swarm_task_queue_length`, it may incorrectly trigger global cloud alerts or hide actual multi-tenant queue degradation.

To resolve this, we must inject `deployment_mode` into all queue-related metrics, deriving it via the `OHC_MULTITENANT` environment variable as documented in our architectural guidelines.

## Design Doc
**High-Level Architecture:**
- **Entity Types:** Telemetry Events, Queue Gauges, Sub-Agent Task Counters.
- **Key Relationships:** Swarm task queues and sub-agent queues interact with the OpenTelemetry bridge, but their current payloads lack environment context. We will enrich these metric calls.
- **Integration Points:** The telemetry recording functions will accept an additional context parameter (or dynamically evaluate the environment) to inject `deployment_mode` alongside the `metric.WithAttributes` OpenTelemetry API call.
- **AI Agent Integration Points:** Agent Swarm monitoring heavily relies on these gauges to determine when to scale workers or apply self-correction. The tagged metrics will allow the Business Advisory Department to generate isolated health reports for Standalone users without cloud noise.

**UI/UX Considerations:**
- From an observability perspective, Grafana dashboards (such as `harness_efficiency.json`) will be updated to include `deployment_mode` in their grouping and filtering (`by (deployment_mode)`), providing clean, bifurcated views of queue depths.

## Implementation Prompt
Update the OpenTelemetry metrics for Swarm and Agent task queues to include the `deployment_mode` attribute.
- **User-Facing Outcome:** Swarm operators and observability dashboards can filter queue length and enqueue/dequeue latencies by `Cloud` or `Standalone` modes. Standalone users will have accurate, isolated local telemetry data.
- **Critical User Journey (CUJ):** When an AI Agent enqueues a new background task, the telemetry system records the exact queue depth and labels it with the current deployment environment. The operator views the Grafana dashboard and sees distinct, isolated queue metrics for multi-tenant vs. local nodes.
- **Acceptance Criteria:**
  - `RecordSwarmTaskQueueLength`, `RecordTaskQueueLength`, and `RecordSubAgentQueueDelay` must include a `deployment_mode` attribute in their OpenTelemetry recording calls.
  - The `deployment_mode` must be dynamically derived based on the platform's multi-tenant configuration (`OHC_MULTITENANT` environment variable).
  - All existing unit tests in `telemetry_test.go` and `buffer_test.go` must be updated to pass with the new parameters. E2E tests must verify that metrics are emitted with the correct attributes without causing panics.

## Priority
P1

## Estimated Scope
Small
