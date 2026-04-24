# OHC Hybrid Telemetry & Swarm Observability Assessment

## Problem Statement
The OHC platform operates in a hybrid deployment model, featuring both Cloud-native (multi-tenant K8s) and Standalone (local execution) modes. Currently, there is a lack of deep understanding and unified visibility regarding how our AI Swarm performs across these environments. Discrepancies in agent job execution throughput, response latency, database query times, and lock contention between Cloud and Standalone modes are obscure. This creates blind spots for swarm health and cost-efficiency optimization, specifically identifying anomalous tenant usage or queue depth issues. We lack a comprehensive gap analysis and concrete strategy for unifying our telemetry pipeline to fully support OHC’s radically simple, AI-first promise.

## Research Report
1. **Telemetry Infrastructure Review**:
   - The backend utilizes OpenTelemetry for traces and Prometheus for metrics (`src/server/telemetry`).
   - `telemetry.go` handles initialization. Metrics are exported across specific files (`metrics.go`, `mcp_metrics.go`, `rag_sync_metrics.go`, `minimax_metrics.go`). In standalone mode (`OHC_STANDALONE=true`), telemetry can be disabled for local sovereignty, requiring careful explicit context labeling when enabled.
   - Various Grafana dashboards exist (e.g., `hybrid-telemetry.json`, `kairos_hybrid_metrics.json`), using the OHC premium design system (Glassmorphism, Outfit/Inter fonts), but they currently do not provide a unified side-by-side comparison of Cloud vs. Standalone performance with tenant-level granularity.
2. **Identified Bottlenecks & Gaps**:
   - *Job Queue Depth & Locking*: In Cloud mode, PostgreSQL `SKIP LOCKED` and Redis Redlock handle concurrency. In Standalone, SQLite and localized locking mechanisms handle this. The performance differential in job dequeue rates and lock contention under load is unquantified in existing telemetry.
   - *Agent Response Latency*: Network partition behavior and LLM provider latency can manifest differently in standalone vs. cloud environments.
   - *Cost Efficiency & Tenant Anomaly*: Lack of granular per-tenant AI token usage vs. storage size vs. API call volume visualization in a unified dashboard to proactively flag runaway AI agent loops.
3. **Swarm Health Evaluation**:
   - The platform needs explicit tracking for "Missions Started" vs. "Missions Completed/Stuck".
   - The synchronization daemons need explicit throughput comparisons across deployment models.

## Design Doc
### High-Level Architecture Additions
- **Enhanced Prometheus Metrics**: Introduce explicit labels for `deployment_mode` (cloud/standalone) and `tenant_id` to existing critical metrics (queue depth, AI token usage, API latency).
- **Tenant Cost Metering Service**: A scheduled job to aggregate LLM token usage, GCS/MinIO storage bytes, and API call volumes per `tenant_id`, emitting these as summarized Prometheus metrics.
- **Unified Hybrid Dashboard**: Create a new Grafana dashboard (`ohc-swarm-hybrid-health.json`) focusing on Side-by-Side comparison of Cloud vs Standalone metrics for:
  - Agent Task Completion Rate
  - PostgreSQL `SKIP LOCKED` Dequeue Latency vs. SQLite Dequeue Latency
  - Redis Redlock Contention Rate
  - Top 10 Costliest Tenants (Tokens + Storage)

### UI/UX Flow (Dashboard View)
- The OHC internal operator portal (375px mobile-first) will feature a "Swarm Health" tab pulling summarized Grafana panels.
- Visual breakdown:
  - Global Agent Success Rate (Donut Chart)
  - Latency Histograms (Cloud vs Standalone side-by-side)
  - Anomalous Tenant Alerts List.

## Implementation Prompt
**Objective**: Implement unified observability enhancements to support hybrid (Cloud vs. Standalone) Swarm monitoring and per-tenant cost analysis.

**Tasks**:
1. **Extend Metrics Context**: Update the Prometheus metric registration in `src/server/telemetry/metrics.go` (and related files) to consistently inject a `deployment_mode` (cloud/standalone) label and `tenant_id` label where applicable (especially for LLM token usage and DB query latency).
2. **Dashboard Creation**: Create a new Grafana dashboard JSON definition `deploy/docker/grafana/provisioning/dashboards/ohc-swarm-hybrid-health.json` and ensure it is also reflected in `deploy/helm/ohc/dashboards/` (if Helm is used). It must visualize queue depths, agent response latency (split by deployment mode), and top 10 costliest tenants.
3. **Test Verification**: Implement unit tests in `src/server/telemetry/telemetry_test.go` to assert that metrics are correctly tagged with `deployment_mode` and `tenant_id` when the context provides them.

**Acceptance Criteria**:
- Prometheus `/metrics` endpoint exposes metrics with `deployment_mode` and `tenant_id`.
- The new Grafana dashboard JSON is structurally valid.
- Unit tests (`bazelisk test //src/server/telemetry/...`) pass with 100% coverage on new code.
- No direct implementation of complex new business logic; just telemetry plumbing.

## Priority
P1

## Estimated Scope
Medium
