<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title
Hybrid Telemetry Review: Swarm Cost Efficiency and Health Analysis

# Problem Statement
The OneHumanCorp (OHC) platform operates agent swarms seamlessly across Cloud-native (multi-tenant K8s) and Standalone (local desktop) contexts. While general API latencies and overall token usage are tracked, the platform lacks cohesive, mode-aware telemetry that directly correlates task execution behavior with resource consumption and potential inefficiencies. Specifically, in Standalone mode, SQLite's single-writer constraints can introduce lock contention when multiple AI agents attempt to coordinate, leading to hidden retries and inflated token burn rates. Non-technical business owners and swarm operators lack human-readable insights to diagnose these mode-specific bottlenecks. Without granular visibility into job queue processing latency, lock contention rates, and their impact on per-tenant AI usage, it is impossible to identify anomalous "stuck" agents or accurately assess the true cost efficiency of the Swarm across different deployment modes.

# Research Report
1. **Hybrid Telemetry Review:** Analysis of K8s logs, production databases, and existing metrics reveals fragmented observability. Current metrics track overall token usage and some state transitions but fail to capture the latency delta between PostgreSQL queue mechanisms (Cloud) and SQLite single-writer limits (Standalone).
2. **Observability Gap Analysis:** The platform lacks comprehensive metric tagging with the `deployment_mode` attribute across critical swarm execution events. Key missing metrics include: `ohc_queue_processing_duration_seconds` (Histogram for job queue latency), `ohc_task_queue_depth` (Gauge for pending and dead-letter jobs), and `ohc_database_lock_contention_total` (Counter for retry events).
3. **Bottleneck Hunting:** A primary bottleneck identified is lock contention during hybrid state synchronization in Standalone mode. The SQLite mutex mechanism hits retry exhaustion during high swarm activity, contrasting with the Cloud's efficient `FOR UPDATE SKIP LOCKED`.
4. **Swarm Health Assessment:** The health of the swarm is obscured by the inability to distinguish between standard task execution and prolonged retries caused by queue contention. This makes it difficult to detect stuck missions or excessive dead-letter accumulation.
5. **Cost Efficiency Analysis:** Tenants with stuck agents in Standalone mode may experience repeated retry loops, leading to anomalously high AI token burn rates. Correlating task execution duration and queue processing latency with per-tenant cost metering is essential to halt runaway agent loops.

# Design Doc
- **Telemetry Aggregation Framework:** Standardize the emission of mode-aware metrics. Implement a global OpenTelemetry interceptor or update metric initializations in `srcs/server/telemetry/telemetry.go` to automatically inject the `deployment_mode` attribute (derived via `kairos.GetMode()`) into all relevant metrics.
- **Core Entities & Instrumentation:**
  - *Swarm Job Latency:* Introduce `ohc_queue_processing_duration_seconds` to track end-to-end duration from job enqueue to agent completion, tagged by mode and queue type.
  - *Task Queue Depth:* Introduce `ohc_task_queue_depth` to monitor pending and dead-letter swarm events across both Postgres and SQLite.
  - *Contention Counter:* Introduce `ohc_database_lock_contention_total` to track the rate of lock retry events (e.g., SQLite lock contention), tagged by mode and DB type.
- **Dashboards Structure:**
  - Create a unified "Hybrid Swarm Health & Cost Efficiency" dashboard in Grafana.
  - Visualizations must juxtapose Cloud vs. Standalone performance side-by-side, including: Task Queue Depth, Job Processing Latency Distribution, Database Contention vs. Queue Size, and Per-Tenant Cost Metering.
- **AI Agent Integration:**
  - The Business Advisory ("The Advisor") department must be able to consume aggregated task queue metrics and cost data to deliver plain-language reports to the user (e.g., "Your background agents are currently experiencing delays due to high task volume, leading to a slight increase in projected AI costs. Consider pausing non-essential automated tasks.").

# Implementation Prompt
Implement the missing Hybrid Telemetry metrics for Swarm task queue contention and cost efficiency. Introduce OpenTelemetry metrics for Swarm queue depth (`ohc_task_queue_depth`), job processing latency (`ohc_queue_processing_duration_seconds`), and database lock retry rates (`ohc_database_lock_contention_total`), ensuring all are explicitly tagged with the deployment mode using `kairos.GetMode()`. Update the telemetry initialization in `srcs/server/telemetry/telemetry.go` accordingly. Create the corresponding Grafana dashboard JSON configuration (`srcs/server/monitoring/dashboards/hybrid_swarm_health_dashboard.json`) to visualize these metrics side-by-side for Cloud and Standalone modes. Update the Business Advisory agent's context injection to receive summary statistics of the queue health and cost projections so it can draft plain-language warnings if agents are stuck or costs spike. Provide end-to-end tests that simulate a high-throughput swarm task burst in both Cloud and Standalone environments, verifying that the metrics are correctly captured and that the Advisor agent correctly issues a summarized health report.

# Priority
P1

# Estimated Scope
Medium

</div>