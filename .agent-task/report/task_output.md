issue_title: "Add Mode-Specific Prometheus Metrics to Sub-Agent Queue and Spawner"
issue_description: |
  # Title: Add Mode-Specific Prometheus Metrics to Sub-Agent Queue and Spawner

  ## Problem Statement
  The OHC Hybrid Architecture requires deep observability into Sub-Agent queues to distinguish between Cloud-native (multi-tenant K8s) and Standalone (local SQLite) contexts. Currently, existing telemetry metrics like `SubAgentQueueDelayHistogram` and `TaskClaimContentionTotal` lack proper alignment with OHC standard metric naming schemes and deployment mode tagging. Furthermore, sub-agent spawn error rates are not explicitly tracked with mode context in the worker pool or orchestration queues. This prevents full-spectrum observability of bottlenecks like queue latency differences, SQLite vs Postgres lock contention, and sub-agent spawn error rates.

  ## Research Report
  The underlying queues are implemented using PostgreSQL (`PgTaskQueue` in `pg_queue.rs`), SQLite (`SQLiteTaskQueue` in `sqlite_queue.rs`), and a generic `QueueManager` in `queue.rs`. Workers fetch tasks from these queues and failures in execution or polling occur in paths within `worker_pool.rs` and `queue.rs`.
  By defining explicit OpenTelemetry metrics grouped by `mode`, we can natively map Cloud vs. Standalone performance disparities in Grafana. The metric names need to be strictly defined as `ohc_sub_agent_queue_latency_seconds` (Histogram), `ohc_sub_agent_spawn_errors_total` (Counter), and `ohc_sub_agent_lock_contention_total` (Counter).

  ## Design Doc
  1.  **Define Prometheus Metrics in Telemetry Package:**
      *   Update `get_sub_agent_queue_delay_histogram()` to emit `ohc_sub_agent_queue_latency_seconds` (Histogram). This should tag the attribute as `mode` instead of `deployment_mode`.
      *   Update `get_task_claim_contention_total()` to emit `ohc_sub_agent_lock_contention_total` (Counter). Ensure the attribute `mode` tracks whether lock contention happens in `sqlite` or `postgres` contexts.
      *   Add a new Counter: `ohc_sub_agent_spawn_errors_total` to track spawn and execution failures.

  2.  **Code Updates (`src/server/telemetry/mod.rs` & `queue.rs` & `orchestration/queue/`):**
      *   **`telemetry/mod.rs`:** Update naming for latency and contention metrics. Create `record_sub_agent_spawn_error(mode: &str)`. Update parameter usages.
      *   **`orchestration/queue/pg_queue.rs` & `sqlite_queue.rs`**: In `dequeue()`, when `start_poll.elapsed() > 100ms`, invoke `record_task_claim_contention("postgres")` or `record_task_claim_contention("sqlite")` respectively. Pass the appropriate `mode` to `record_sub_agent_queue_delay()`.
      *   **`orchestration/queue/worker_pool.rs`**: Ensure that panics, job handler errors, timeouts, or failure to register a fail state in `WorkerPool::new_with_timeout` trigger `record_sub_agent_spawn_error(::server_telemetry::get_deployment_mode())`.
      *   **`queue.rs` (`QueueManager`)**: Ensure that when `retry_count > 3` during polling, lock contention metrics are emitted. Additionally, if the job handler returns an error or times out in `start_polling()`, emit the `record_sub_agent_spawn_error`.

  3.  **Grafana Visualization:**
      *   Update `deploy/docker/grafana/provisioning/dashboards/ohc-kairos-hybrid.json` (and other hybrid dashboards if needed) to visualize:
          *   Sub-Agent Queue Latency (P95) by Mode using `ohc_sub_agent_queue_latency_seconds`.
          *   Sub-Agent Spawn Error Rate by Mode using `ohc_sub_agent_spawn_errors_total`.
      *   Dashboards must follow the premium aesthetic (transparent Glassmorphism, Outfit/Inter typography).

  ## Implementation Prompt
  You are an Implementer. Implement the sub-agent telemetry improvements as designed above:
  1.  Update `src/server/telemetry/mod.rs` to rename existing queue metrics to `ohc_sub_agent_queue_latency_seconds` and `ohc_sub_agent_lock_contention_total`. Ensure the attributes are labelled exactly as `mode`. Add the new `ohc_sub_agent_spawn_errors_total` Counter and its helper function `record_sub_agent_spawn_error`.
  2.  Update `src/server/orchestration/queue/pg_queue.rs` and `src/server/orchestration/queue/sqlite_queue.rs` to pass exact modes (`"postgres"` or `"sqlite"`) to `record_task_claim_contention`.
  3.  Update `src/server/orchestration/queue/worker_pool.rs` and `src/server/queue.rs` to capture execution and spawn errors using the new `record_sub_agent_spawn_error` telemetry function.
  4.  Update the Grafana dashboards in `deploy/docker/grafana/provisioning/dashboards/` (e.g., `ohc-kairos-hybrid.json`) to visualize these mode-labeled metrics natively inside Text/HTML panels conforming to OHC styling guidelines.
  5.  Ensure all tests pass and achieve 100% test coverage using `bazel test //...`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
