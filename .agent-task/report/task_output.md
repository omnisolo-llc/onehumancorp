issue_title: "[no-work finding] Latency and baseline metrics for billing and AI budget checks"
issue_description: |
  # Latency and baseline metrics for billing and AI budget checks

  **Role:** Principal Performance Engineer & Bolt (L7)

  ## Investigation
  I audited the codebase to establish workload-specific baseline latency, attempted/completed outcomes, and resource cost for the business workflow involving provider waits and reserved capacity, specifically focusing on the new usage API and budget reservations.

  **Specific files checked:**
  - `src/server/api/usage_api.rs`
  - `src/server/api/billing_api.rs`
  - `src/server/pricing/budget.rs`
  - `src/server/pricing/cost_aggregator.rs`

  **Existing caching strategies:**
  - `COST_DASHBOARD_CACHE` and `MY_PLAN_CACHE` use `HybridCache` to avoid database reads on every dashboard load.
  - `DAILY_COST_CACHE` and `AGENT_COST_CACHE` in `cost_aggregator.rs` use in-memory `OnceLock<Mutex<HashMap>>` with a 300-second (5 min) TTL, reducing DB load for telemetry aggregation.

  **Lack of baseline metrics / Next steps:**
  - While there are robust caching layers, there is a lack of production baseline metrics for the p50/p95/p99 latencies of the budget reservation system under load, especially when handling concurrent token streaming with BYOK and OHC-funded inference.
  - `BudgetManager` uses a synchronous `Mutex` for its state (`state: Arc<Mutex<BudgetState>>`), which might become a bottleneck under high concurrent load for a single tenant.
  - Parallel execution in `my_plan_handler` and `cost_dashboard_handler` is already implemented using `tokio::join!` for aggregating various cost factors, meaning no immediate structural parallelization optimizations are required there.

  **Skill Provenance:**
  Loaded Superpowers skill using-superpowers from revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d.
issue_priority: "Low"
issue_category: "Performance"
issue_type: "Research"
issue_label: "no-work finding"
assignees: ["Bolt"]
