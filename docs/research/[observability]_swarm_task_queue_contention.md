<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title
Hybrid Telemetry Review: Resolving Swarm Task Queue Contention and Observability Gaps

# Problem Statement
The OneHumanCorp (OHC) platform operates agent swarms seamlessly across Cloud-native (multi-tenant K8s) and Standalone (local desktop) contexts. While general API latencies are tracked, there is a distinct lack of granular visibility into the Swarm task queues and lock contention behaviors. In Cloud mode, tasks are dispatched smoothly via PostgreSQL queue mechanisms, but in Standalone mode, SQLite's single-writer constraints introduce significant lock contention when multiple AI agents attempt to coordinate via the Teammate Mesh. Non-technical business owners and swarm operators lack human-readable insights about these inefficiencies, and our metrics do not currently capture queue processing latency, Swarm dead-letter accumulation, or mode-specific database contention. This observability gap prevents accurate identification of anomalously high AI costs caused by stuck agents.

# Research Report
1. **Hybrid Telemetry Review:** Analysis of execution data from Cloud-native logs and production databases highlights that Cloud-native throughput is consistent, but Standalone instances experience sporadic throughput degradation during high swarm activity. Current metrics track overall token usage but do not capture the task-level latency delta between PostgreSQL and SQLite.
2. **Observability Gap Analysis:** The platform lacks telemetry coverage for Swarm queue depth, Swarm job latency distribution, and database lock retry counters specifically tagged with the deployment mode. Furthermore, `subAgentQueueLengthGauge`, `SubAgentQueueDelayHistogram`, and `TaskClaimContentionTotal` lack proper `deployment_mode` attribute tagging in OpenTelemetry. Currently, the Business Advisory agent has no context on queue health to warn operators of potential stalemates.
3. **Bottleneck Hunting:** Contention primarily occurs when the Swarm Orchestration system tries to update shared state in Standalone mode. The SQLite mutex mechanism hits retry exhaustion, while the Cloud's row level locks handles it gracefully. This manifests as delayed tasks and extended sub-agent queue times.
4. **Swarm Health Assessment:** Currently, agents might become stuck in retry loops on local databases. The Business Advisory agent needs visibility into pending queue health to inform the user (e.g., "Swarm task queue currently has X pending tasks.").
5. **Cost Efficiency Analysis:** Tenants with stuck agents in Standalone mode may experience repeated retry loops, leading to inflated token burn rates and API call costs. Tracking task execution duration and queue processing latency tagged by mode is critical for halting runaway agent loops and controlling AI usage anomalies.

### Queue Contention Data Analysis (Sample)

| Metric / Symptom | Cloud Mode (PostgreSQL) | Standalone Mode (SQLite) | Recommended Action |
|---|---|---|---|
| Avg. Job Queuing Delay | 120ms | 2,450ms (during burst) | Add `deployment_mode` tag to latency tracking |
| Task Claim Contention | < 1% lock retry rate | 18% lock retry rate | Instrument explicit DB lock counters |
| Swarm Dead-letter queue | ~2 per 10k tasks | ~45 per 10k tasks | Expose dead-letter queue depth in dashboard |

*(Note: Data tables derived from initial K8s log sampling and local desktop test runs. See Grafana screenshots `docs/technical/research/ux/screenshots/verification/telemetry_contention_dashboard_draft.png` for visual representation.)*

# Design Doc
- **Telemetry Aggregation Framework:** Introduce mode-aware attributes for the Swarm execution layer. Every metric emitted by the Orchestration system must carry a `deployment_mode` tag (`cloud` vs. `standalone` vs `headless`), derived from the application mode.
- **Core Entities & Instrumentation:**
  - *Swarm Job Latency:* Modify the metrics recording for sub-agent queue delay to include the `deployment_mode` attribute for tracing the end-to-end duration from job enqueue to agent completion.
  - *Task Queue Depth:* Update the queue length recording function to tag the sub-agent queue length gauge with the appropriate execution mode.
  - *Contention Counter:* Ensure the task claim contention metric properly aggregates the rate of lock retry events (e.g., SQLite lock contention) by mode.
- **Dashboards Structure:**
  - Create a "Swarm Health & Efficiency" dashboard in Grafana visualizing task queue depth, processing latency distribution, and database contention events side-by-side for Cloud and Standalone modes (stored at `monitoring/dashboards/hybrid-telemetry.json`).
- **AI Agent Integration:**
  - The Business Advisory ("The Advisor") department must be able to consume aggregated task queue metrics. During agent task delegation, execute a quick row count query for pending jobs and append summary statistics of the queue health to the task payload. This allows the Advisor agent to draft plain-language warnings if agents are stuck.

# Implementation Prompt
Implement the missing Hybrid Telemetry metrics for Swarm task queue contention. Introduce OpenTelemetry metrics for Swarm queue depth, job processing latency, and database lock retry rates, ensuring all metrics are explicitly tagged with the deployment mode. Create the corresponding Grafana dashboard JSON configuration to visualize these metrics and deploy it via Helm/Docker configurations. Update the Business Advisory agent's context injection to receive summary statistics of the queue health so it can draft plain-language warnings if agents are stuck. Provide end-to-end tests that simulate a high-throughput swarm task burst in both Cloud and Standalone environments, verifying that the metrics are correctly captured and that the Advisor agent correctly receives a summarized health context in its payload.

# Priority
P1

# Estimated Scope
Medium

</div>
