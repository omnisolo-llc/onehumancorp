issue_title: "Evaluate compute/API charging and BYOK caching/metrics"
issue_description: |
  # Findings

  ## Specific files checked
  - `src/server/services/billing/auditor.rs`
  - `src/server/services/billing/service.rs`
  - `src/server/harness/middleware/usage_ledger.rs`
  - `src/server/harness/middleware/usage_meter.rs`
  - `src/server/harness/middleware/inference.rs`
  - `src/server/pricing/budget.rs`
  - `docs/research/business_capability_and_usage_economics_audit.md`

  ## Existing caching strategies
  - The LLM inference paths actively track `cached_input_tokens` and `cached_input_micros_per_million` inside `usage_ledger.rs` and `usage_meter.rs` to correctly account for provider-level prompt caching without double-billing.
  - However, for application-level data caching, there is no Redis or external caching server in use. The `auditor.rs` utilizes local process-memory structures like `Mutex<HashMap<String, i64>>` for tracking `tenant_cached_tokens`, which does not persist across restarts or scale horizontally.
  - Local caching is exposed as a capability (`LocalServiceKind::Cache`, `cache.read`, `cache.write`) to the harness, but it relies on internal memory.

  ## Lack of baseline metrics
  - There are no established baseline percentiles (p50/p95/p99) for the owner work feed, inference response times, or UI rendering times.
  - The 10-minute CI and 30-minute full build target exist for CI/CD time tracking, but there are no persistent runtime performance or latency metrics stored for business paths.

  ## Compute/API Charging and BYOK
  - The `budget.rs` implementation implements a "soft limit" that increments spending and reports false only *after* exceeding the limit, failing to provide atomic reservation of funds before provider requests are dispatched.
  - Usage accounting in `auditor.rs` remains partially separated from provider metadata; it lacks provider request IDs, which prevents invoice-grade reconciliation against actual model path usage data captured in `harness/middleware/inference.rs`.
issue_priority: "Medium"
issue_category: "Research"
issue_type: "Investigation"
issue_label: ""
assignees: []
