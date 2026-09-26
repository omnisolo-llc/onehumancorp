issue_title: "F05: Telemetry not invoice-grade meter (Blocked)"
issue_description: |
  ## Research Report

  ### Problem Statement
  Finding F05 states: "Current telemetry/cost reports are not an invoice-grade meter" and requires "Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit".
  This relates to the broader initiative of providing an accurate bill for AI agent/department usage within the platform.

  ### Status
  Blocked / No-Work

  ### Blocked Prerequisites
  As per the `RESEARCH.md` and `business_capability_and_usage_economics_audit.md` contracts, implementing a concrete usage billing or price card is blocked until the following evidence is gathered:
  1. **Measured Workloads:** We do not currently have a measured representative serving cost or a representative workload distribution. We need real, permissioned customer workloads with data on active/reserved resources, waiting time, setup effort, and success/failure rates.
  2. **Provider-Invoice Reconciliation:** We need an actual provider invoice reconciliation. Currently, "Current telemetry/cost reports are not an invoice-grade meter". We cannot construct an accurate cost allocator without actual invoice data to verify against.
  3. **Willingness to pay / Owner preference:** The previous $99 subscription hypothesis is suspended. We need to "Test owner preference and unit economics rather than quietly reinstating the old subscription". We cannot dictate pricing logic until real pilot tests yield willingness-to-pay evidence.
  4. **Support Cost Evidence:** A true usage cost includes support and incident handling, not just raw compute. We lack this data for the current models and tools.

  ### Findings & Current State
  - The `BudgetManager` in `src/server/pricing/budget.rs` manages thresholds and limits in integer cents, handling over-limit errors appropriately (Fail closed).
  - The `calculator.rs` handles cost calculations.
  - However, the `CostDashboardResponse` generated in `billing_api.rs` and `lib.rs` uses simple estimated rates rather than durably tracked, reconciled usage events bound to specific provider requests, auth modes, and rates.
  - Implementing an invoice-grade meter requires capturing "durable, deduplicated events with tenant/project/task/attempt and provider request IDs", tracking retries, settling unknown outcomes, and executing an external reconciliation process.
  - Proceeding with code implementation of these billing/cost aggregation pipelines without the blocked prerequisites (real data, reconciliation evidence, established rate-card model) would violate the contract's mandate to avoid inventing pricing mechanisms ahead of evidence.

  ### Superpowers Provenance
  - **Skill:** `using-superpowers`
  - **Revision:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Checks:**
    - Read `skills/using-superpowers/SKILL.md` from the upstream Superpowers repository in a local scratch directory (`.scratch/superpowers`).
    - Recognized the need for evidence-based decision-making as per the `RESEARCH.md` contract.
  - **Outcomes:** Followed the workflow constraints, documented the blocked status accurately without fabricating evidence or code implementation.

issue_priority: "High"
issue_category: "documentation"
issue_type: "task"
issue_label: "ohc:lane:finance"
assignees: []
