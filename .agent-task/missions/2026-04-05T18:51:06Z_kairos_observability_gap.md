---
status: DONE
agent: jules
---

# Title: Implement Hybrid Observability & Metrics for KAIROS Orchestration APIs

## Problem Statement
The recent implementation of the **KAIROS AutoDream vector RAG pipelines** (`/api/v1/autodream/sync`, `/api/v1/autodream/query`) and **Teammate Mesh coordination** APIs has created a significant "Observability Gap".
Currently, these critical endpoints are missing specific OpenTelemetry and Prometheus metric instrumentation. Without granular metrics tracking latency, throughput, and error rates—and differentiating them based on the execution context (Cloud Postgres multi-tenant vs. Standalone SQLite single-user)—it is impossible to effectively orchestrate hybrid architectural optimizations or detect role-specific bottlenecks as required by the OHC Hybrid Telemetry mandate. Additionally, the SSE endpoint `/api/v1/stream` documented in the OHC API Playbook is missing.

## Research Report
**Market & Codebase Findings:**
1. **API Missing:** The `docs/api/playbook.md` explicitly lists `GET /api/v1/stream` as an SSE stream for real-time task changes (`AgentHired`, `TaskCompleted`), but this endpoint is completely absent from `srcs/server/dashboard/server.go`.
2. **Metrics Gap:** Operations on AutoDream (`handleAutoDreamSync`, `handleAutoDreamQuery`) and Teammate Mesh (`handleMeshBroadcast`, `handleMeshDirect`) perform intensive database operations but lack explicit metric counters or duration histograms.
3. **Hybrid Modes Constraint:** Operations in `Cloud-Native` multi-tenant Postgres mode perform differently compared to `Standalone` SQLite mode. We need to expose a `hybrid_mode` label in the OpenTelemetry instrumentation to accurately pinpoint bottlenecks on Grafana dashboards.

## Design Doc
### Architecture Updates
- **Telemetry Module Update:** Add new global metric definitions in `srcs/server/telemetry/telemetry.go` for `ohc_autodream_sync_duration_seconds`, `ohc_mesh_broadcast_total`, and `ohc_hybrid_rag_latency`.
- **API Handler Wrappers:** Modify `srcs/server/dashboard/server.go` to inject Prometheus instrumentation wrappers into the AutoDream and Mesh endpoint handlers. Ensure that the `deployment_mode` (Cloud vs. Standalone) is appended as a label to all Prometheus vectors.
- **SSE Stream Implementation:** Implement the missing `handleStream` in `srcs/server/dashboard/server.go` to push SSE events based on a global event broker.

### UI / Grafana Wireframes (Visual Excellence)
Any internal UI representing these metrics must adhere to the OHC-SIP Stylistic Intent Profile:
```html
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">
  <h2>KAIROS Orchestration Vitality</h2>
  <div style="display: flex; gap: 10px;">
      <div style="border: 1px solid rgba(0, 255, 0, 0.2); padding: 10px; border-radius: 8px;">Cloud RAG Latency: 45ms</div>
      <div style="border: 1px solid rgba(0, 255, 0, 0.2); padding: 10px; border-radius: 8px;">Local RAG Latency: 12ms</div>
  </div>
</div>
```

## Implementation Prompt
> "You are an Implementer agent. Your task is to close the observability gap for the KAIROS Orchestration system.
> 1. In `srcs/server/dashboard/server.go`, implement the missing `GET /api/v1/stream` Server-Sent Events endpoint to stream real-time orchestration events.
> 2. In `srcs/server/telemetry/telemetry.go` (or a specific new file `srcs/server/telemetry/kairos_metrics.go`), define new Prometheus metrics: `ohc_autodream_sync_duration_seconds` (Histogram), `ohc_autodream_query_duration_seconds` (Histogram), and `ohc_mesh_broadcast_total` (Counter). Ensure these metrics include a `deployment_mode` label (values: `cloud`, `standalone`).
> 3. Update the HTTP handlers for `/api/v1/autodream/sync`, `/api/v1/autodream/query`, and `/api/mesh/broadcast` in `srcs/server/dashboard/server.go` to record these metrics upon execution.
> 4. Add unit tests for the SSE stream handler and ensure the telemetry logic passes `bazelisk test //...`.
> Ensure all Go code is properly formatted."

## Priority
`P1` (High)

## Estimated Scope
Medium
