issue_title: "Implement true BYOK vs Managed API telemetry and cost reconciliation"
issue_description: |
  **Title**: Implement true BYOK vs Managed API telemetry and cost reconciliation
  **Problem Statement**: The current product telemetry implementation uses integer counters for `mission_cost_cents` and `byok_cost_counter` but lacks invoice-grade reconciliation. Customers paying for BYOK inference are not sufficiently isolated from OHC's managed inference debit. A robust cost model needs exact usage tracking (CPU, memory, storage, browser capacity, payment collection) separate from provider-invoiced API keys to support dynamic per-workflow pricing securely.

  **Research Report**:
  - Codebase Analysis: `src/server/harness/middleware/usage_ledger.rs` and `src/server/harness/middleware/usage_meter.rs` implement basic tracking. `src/server/pricing/budget.rs` and `src/server/pricing/rate_limit.rs` call `store.mission_cost_cents.add` and `store.byok_cost_counter.add`.
  - Current Behavior: The usage ledger logs unknown outcomes correctly, but lacks comprehensive bounds on BYOK direct costs vs OHC hosted costs. The `Tracker` in `billing.rs` provides token summaries, but not reconciled multi-resource serving cost totals (e.g. queue time, network egress).
  - Provider Terms: Both OpenAI and Anthropic require distinct tracking for consumer subscriptions versus API keys. True native-client subscriptions are not currently fully brokered.

  **Design Doc**:
  - Architecture: Ensure `PayerMode::ByokApi` bypasses OHC debit completely while recording for customer visibility. Enhance `ViolationStore` and `Tracker` to ingest separate compute/storage/network metrics, combining them into a final `ServingCost` report decoupled from BYOK LLM costs.
  - Mobile UX: No direct UI change required for the background worker, but the dashboard must explicitly label BYOK cost differently than OHC compute cost on a 375px display, using a clean card layout hiding technical details behind an "Advanced Settings" switch.
  - Security/Isolation: Ensure multi-tenant Zero Trust boundaries are maintained when calculating idle vs active shared resource attribution.

  **Implementation Prompt**: Implement an isolated metering pipeline that distinctly separates `PayerMode::ByokApi` costs from `PayerMode::ManagedApi` costs across all resource types (compute, network, storage, LLM), outputting a reconciled invoice-ready serving cost. Ensure tests explicitly verify BYOK usage never decrements OHC budgets and that failed tasks are not billed as successful customer revenue.
  **Priority**: P1
  **Estimated Scope**: Large
  **Strategy Admission**: Target OHC-09/10 measurement phase; targets non-technical owner evaluating real usage margins; non-goals include generic ERP implementation or multi-tenant database re-write; expects bounded happy/failure path checks without real provider billing.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
