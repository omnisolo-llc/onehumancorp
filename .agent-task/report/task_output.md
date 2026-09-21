issue_title: "Enable Tenant Usage Isolation and BYOK for Telemetry/Cost Reporting"
issue_description: |
  # Research Report: Cost Accounting and Usage Evidence

  ## 1. Problem Statement
  Currently, `omnisolo` tracks telemetry and usage metrics through `ViolationStore` and `BudgetManager`. The system needs to accurately report on an invoice-grade meter. Finding `F05` from the capability audit requires:
  > Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit

  The system needs to distinguish between "OHC-funded" execution vs. "customer BYOK API" inference execution, preventing double-charging of resources.

  ## 2. Research Report
  - **F05 Requirement**: The audit found that current telemetry/cost reports are not an invoice-grade meter. "The owner should see estimated task cost and maximum authorized spend, then an itemized OHC bill, separate customer-direct provider usage, and clear unknown or pending reconciliation."
  - **BudgetManager (src/server/pricing/budget.rs)** uses atomic limits to enforce safety, and pushes telemetry using OpenTelemetry metrics (`llm_cost_counter`, `mission_cost_cents`).
  - **Gap**: There is no distinction of payer (BYOK vs OHC). `BudgetManager` simply decrements its own bounds and emits telemetry.

  ## 3. Design Doc
  **Architecture**:
  - `BudgetManager` and related telemetry should explicitly accept an indicator of whether the usage is `BYOK` or `OHC_FUNDED`.
  - When `BYOK` is used, the cost should NOT decrement the core OHC monetary budget in the same way, but the telemetry still needs to track it as `customer-direct provider usage`.
  - The telemetry output should append a new label: `payer_mode` (`byok` or `ohc`).

  ```mermaid
  graph TD
      A[Agent Inference] -->|Cost incurred| B(BudgetManager)
      B --> C{Payer Mode?}
      C -->|OHC Funded| D[Decrement OHC Balance]
      C -->|BYOK| E[Skip Balance Decrement]
      D --> F[Emit OHC Telemetry]
      E --> G[Emit BYOK Telemetry]
  ```

  ## 4. Implementation Prompt
  - Update `BudgetManager::record_spend_cents` to distinguish `payer_mode`.
  - Allow passing `byok: bool` to spending functions or create a `record_byok_spend` path that emits telemetry but bypasses the OHC budget limit reduction.
  - Implement comprehensive tests for this new behavior.

  ## 5. Metadata
  Priority: P1
  Estimated Scope: Medium
  Strategy Admission: OHC-05; Run stage; observed gap from audit; durable idempotent usage; no duplicate BYOK debit.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
