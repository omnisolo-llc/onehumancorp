# Title: Proactive Hybrid Observability Audit & Teammate Mesh Metrics Injection

## Problem Statement
The OHC Hybrid Architecture seamlessly shifts workloads between Cloud-native (multi-tenant K8s, pgvector) and Standalone (local SQLite) contexts. However, observational data indicates critical blind spots in how inter-agent communication (Teammate Mesh) and distributed state transitions are tracked across these environments. In Cloud mode, we lack granular Prometheus metrics for cross-tenant RPC overhead, while in Standalone mode, synchronous buffer fallbacks drop critical high-fidelity latency events. This asymmetric observability prevents the "AutoDream" consolidation loop from accurately restructuring agent configurations based on performance bottlenecks.

## Research Report
- **Hybrid Context Constraints:** According to `CLAUDE_OHC.md` and OHC-SIP, Cloud-Native mode routes metrics dynamically, while Standalone operates synchronously.
- **Telemetry Missing Keys:** A sweep of `srcs/server/telemetry/telemetry.go` reveals that `MeshLatencyRecorder` and `AgentTransitionLatency` are instantiated, but they lack distinct attributes differentiating "Cloud" vs "Standalone" routing times, making Grafana aggregations impossible to segment.
- **AutoDream Disconnect:** The Vector DB (pgvector) relies on precise performance metrics to prune inefficient sub-agent orchestration trees. If latency histograms aren't capturing standalone overhead versus cloud tenant contention, AutoDream may optimize for the wrong topology.
- **Teammate Mesh Analysis:** Inter-agent mailboxes via Redis Pub/Sub in Cloud mode vs. SQLite mutexes in Standalone mode cause massive latency jitter, yet the metrics pipeline treats both identically.

## Design Doc
1. **Metrics Injection Strategy:** We must update the Teammate Mesh telemetry wrappers in `srcs/server/telemetry/telemetry.go` to inject a `deployment_mode` attribute (Cloud/Standalone).
2. **Buffer Degration Fallback:** Ensure that if `telemetry.BufferMetricFunc` is nil (Cloud-Native Mode), synchronous endpoints gracefully degrade.
3. **Grafana Alignment:** Metrics should map perfectly to OHC Visual Excellence Mandates, allowing developers to query `ohc_mesh_latency{deployment_mode="cloud"}` versus `ohc_mesh_latency{deployment_mode="standalone"}`.
4. **Architectural Consolidation:** These findings and schema requirements will be recorded in `.agent-task/memory/{timestamp}.yml` to inform the overarching AutoDream vector context.

## Implementation Prompt
**Objective:** Enhance Teammate Mesh and Swarm telemetry by injecting Hybrid deployment mode context into critical latency histograms.

**Specific Tasks:**
1. Open `srcs/server/telemetry/telemetry.go`.
2. Locate `RecordMeshLatency`. Currently it records `MeshLatencyRecorder.Record(ctx, latency.Seconds(), metric.WithAttributes(attribute.String("operation", operation)))`.
3. Update `RecordMeshLatency` to accept a `mode string` parameter (or determine it contextually if exposed globally), and add `attribute.String("deployment_mode", mode)` to its attributes.
4. Locate `RecordAgentTransitionLatency`. Update it similarly to include a `mode string` parameter and the `deployment_mode` attribute.
5. Search the codebase (`grep -r "RecordMeshLatency" srcs/server/`) to update all callsites to pass the current OHC_MODE (usually checking `config.IsCloudMode()` or passing a string literal based on context).
6. Ensure that if `BufferMetricFunc` is `nil` in Cloud-Native mode, we still route directly to OpenTelemetry.
7. Run `bazelisk test //srcs/server/telemetry/...` and fix any failing unit tests in `telemetry_test.go` or `telemetry_extra_test.go` by updating the mocked arguments.

**Acceptance Criteria:**
- `RecordMeshLatency` and `RecordAgentTransitionLatency` explicitly track "deployment_mode".
- All tests pass (`bazelisk test //...`).
- OHC-SIP (Stylistic Intent Profile) is respected. No temporary scripts are left in the repo root.

## Priority
P0

## Estimated Scope
Medium
