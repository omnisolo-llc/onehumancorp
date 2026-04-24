<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title
Hybrid Telemetry Review: Resolving Swarm Task Queue Contention and Observability Gaps

# Problem Statement
The OneHumanCorp (OHC) platform operates agent swarms seamlessly across Cloud-native (multi-tenant K8s) and Standalone (local desktop) contexts. While general API latencies are tracked, there is a distinct lack of granular visibility into the Swarm task queues and lock contention behaviors. In Cloud mode, tasks are dispatched smoothly via PostgreSQL queue mechanisms, but in Standalone mode, SQLite's single-writer constraints seem to introduce significant lock contention when multiple AI agents attempt to coordinate via the Teammate Mesh. Non-technical business owners and swarm operators lack human-readable insights about these inefficiencies, and our metrics do not currently capture queue processing latency, Swarm dead-letter accumulation, or mode-specific database contention. This observability gap prevents accurate identification of anomalously high AI costs caused by stuck agents.

# Research Report
1. **Hybrid Telemetry Review:** Analysis of K8s logs and production databases highlights that Cloud-native throughput is consistent, but Standalone instances experience sporadic throughput degradation during high swarm activity. Current metrics track overall token usage but do not capture the task-level latency delta between PostgreSQL and SQLite.
2. **Observability Gap Analysis:** The platform lacks telemetry coverage for Swarm queue depth, Swarm job latency distribution, and database lock retry counters specifically tagged with the deployment mode. Currently, the Business Advisory agent has no context on queue health to warn operators of potential stalemates.
3. **Bottleneck Hunting:** Contention primarily occurs when the Swarm Orchestration system tries to update shared state in Standalone mode. The SQLite mutex mechanism hits retry exhaustion, while the Cloud's `FOR UPDATE SKIP LOCKED` handles it gracefully.
4. **Cost Efficiency Analysis:** Tenants with stuck agents in Standalone mode may experience repeated retry loops, leading to inflated token burn rates. By tracking task execution duration and queue processing latency, we can flag anomalous behavior and halt runaway agent loops.

# Design Doc
- **Telemetry Aggregation Framework:** Introduce mode-aware metrics for the Swarm execution layer. Every metric emitted by the Orchestration system must carry a `deployment_mode` tag (Cloud vs. Standalone).
- **Core Entities & Instrumentation:**
  - *Swarm Job Latency:* Track the end-to-end duration from job enqueue to agent completion.
  - *Task Queue Depth:* Gauge tracking pending swarm events in the primary queue and the dead-letter queue.
  - *Contention Counter:* Track the rate of lock retry events (e.g., SQLite lock contention).
- **Dashboards Structure:**
  - Create a "Swarm Health & Efficiency" dashboard in Grafana visualizing task queue depth, processing latency distribution, and database contention events side-by-side for Cloud and Standalone modes.
- **AI Agent Integration:**
  - The Business Advisory ("The Advisor") department must be able to consume aggregated task queue metrics to deliver plain-language reports to the user (e.g., "Your background agents are currently backlogged due to high task volume, but no action is needed.").

# Implementation Prompt
Implement the missing Hybrid Telemetry metrics for Swarm task queue contention. Introduce OpenTelemetry metrics for Swarm queue depth, job processing latency, and database lock retry rates, ensuring all metrics are explicitly tagged with the deployment mode. Create the corresponding Grafana dashboard JSON configuration to visualize these metrics. Update the Business Advisory agent's context injection to receive summary statistics of the queue health so it can draft plain-language warnings if agents are stuck. Provide end-to-end tests that simulate a high-throughput swarm task burst in both Cloud and Standalone environments, verifying that the metrics are correctly captured and that the Advisor agent correctly issues a summarized health report.

# Priority
P1

# Estimated Scope
Medium

</div>