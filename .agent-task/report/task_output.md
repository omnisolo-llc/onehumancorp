# Research Report: Hybrid Telemetry & OHC Swarm Orchestration Efficiency

**Author**: Principal Data Scientist - Agentic Operations (L7)
**Date**: 1776823959

## Problem Statement
The OHC (OneHumanCorp) swarm operates in two primary environments: a high-concurrency Cloud mode (backed by PostgreSQL row-level locks) and a Standalone mode (backed by SQLite transactions). We currently lack granular visibility into how these two operational modes perform against each other, particularly regarding task throughput, error rates, queue depth, lock contention, and overall system bottlenecks. A deep dive into the `telemetry` module and the deployed Grafana dashboards reveals specific observability gaps that prevent accurate profiling of agent efficiency and cluster health.

## Telemetry Audit Findings

### 1. Hybrid Mode Metric Coverage
- **Current State**: The `telemetry` module instruments various dimensions of system load including `ohc_sqlite_lock_contention_total`, `ohc_sqlite_retry_exhausted_total`, `ohc_postgres_lock_contention_total`, and `ohc_postgres_retry_exhausted_total`.
- **Gap**: There is no top-level metric directly comparing overall throughput explicitly tagged by `deployment_mode` (i.e., `cloud` vs. `standalone`). Dashboards like `hybrid-telemetry.json` attempt to display hybrid metrics but rely on disjoint queries.
- **Agent Transitions**: Metrics like `ohc_kairos_transitions_total` and `ohc_kairos_transition_duration_seconds_bucket` track state transition events by mode, which is excellent, but we need deeper insight into *why* transitions stall or error out based on the underlying datastore.

### 2. Task Orchestration Bottlenecks
- **Cloud (PostgreSQL)**: Utilizing `FOR UPDATE SKIP LOCKED` effectively resolves worker collision and enables horizontal scaling. The primary potential bottleneck observed in metrics is `ohc_postgres_lock_contention_total` combined with `ohc_task_processing_latency_seconds`.
- **Standalone (SQLite)**: Standalone desktop users face entirely different contention dynamics. The single-writer nature of SQLite leads to higher `ohc_sqlite_lock_contention_total` and `ohc_sqlite_retry_exhausted_total`.
- **Insight**: SQLite contention spikes when the AutoDream pipeline executes large semantic ingestion alongside real-time sub-agent tracking. The single WAL file bottleneck causes high latency for fast, small status updates.

### 3. Missing Grafana Dashboards & Visualization Gaps
- **Current Dashboards**: We have `hybrid-telemetry.json`, `infra-observability.json`, and `kairos_hybrid_metrics.json`.
- **Gap**: While we track the data, we do not have a dedicated panel highlighting the direct ratio of Cloud task completions to Standalone task completions per unit time (Throughput Ratio). We also lack a visualization of the "Sub-Agent Queue Delay" (`ohc_sub_agent_queue_delay_seconds`), which would directly expose the end-user wait time.

### 4. Cost & Token Efficiency
- The metric `ohc_agent_token_usage_total` tracks tokens per model/role. In Standalone mode, large context payloads (especially the Omni-Context Grounding) could be severely expensive if falling back to Cloud inference models when local `ollama` or `minimax` fails. The lack of robust local token forecasting might lead to unexpected user costs.

## Design Recommendations

### 1. Instrument `ohc_cloud_vs_standalone_throughput`
Introduce a unified histogram or counter specifically designed to track the *rate of successful mission completions* strictly partitioned by an explicit `deployment_mode` label. This normalizes the comparison between the massive Cloud multi-tenant PostgreSQL array and individual user SQLite instances.

### 2. SQLite Batch Optimization for Standalone Mode
To alleviate `ohc_sqlite_retry_exhausted_total`, we should implement a coalesced batch-update mechanism for agent state transitions in Standalone mode. Instead of each sub-agent immediately committing its status (e.g., `STARTED`, `IN_PROGRESS`, `DELIBERATING`), the `CentrifugeNode` mesh should buffer non-critical state updates and flush them to SQLite in a single transaction, reducing lock contention by an order of magnitude.

### 3. Dedicated Hybrid Health Grafana Panel
Create a new panel in `kairos_hybrid_metrics.json`:
- **Title**: Standalone vs Cloud Contention Ratio
- **Query**: Compare the rate of `ohc_sqlite_lock_contention_total` against `ohc_postgres_lock_contention_total` (normalized per active tenant/worker).

## Proposed Issue Brief

```yaml
title: "Implement Standalone SQLite Batching and Hybrid Throughput Metrics"
problem_statement: "Standalone mode users experience AI agent slowdowns due to SQLite lock contention when multiple sub-agents report status simultaneously. Additionally, swarm operators lack a unified metric comparing Cloud vs. Standalone task throughput, hindering hybrid performance tuning."
research_report: "Audit of 'srcs/server/telemetry' and 'deploy/docker/grafana' reveals high SQLite retry exhaustion ('ohc_sqlite_retry_exhausted_total') during parallel sub-agent execution. Standalone databases cannot handle the same concurrent write patterns as PostgreSQL. Furthermore, while we have discrete datastore metrics, we lack a top-level 'deployment_mode' throughput aggregate."
design_doc: |
  1. Modify the KAIROS Orchestrator to buffer non-critical agent state transitions (e.g., 'THINKING', 'DELIBERATING') in memory when operating in Standalone mode.
  2. Flush these state updates to SQLite in a single batched transaction every 500ms, drastically reducing write-lock acquisition attempts.
  3. Introduce a new OpenTelemetry counter in 'srcs/server/telemetry': 'ohc_hybrid_mission_throughput_total' with a 'deployment_mode' label.
  4. Add corresponding panels to 'kairos_hybrid_metrics.json' to visualize the new throughput metric and SQLite contention reduction.
implementation_prompt: "Implement the Standalone SQLite state transition batcher. Add the 'ohc_hybrid_mission_throughput_total' metric and update the Grafana dashboards to reflect it. Ensure backward compatibility with Cloud mode PostgreSQL operations."
priority: "P1"
estimated_scope: "Medium"
```
