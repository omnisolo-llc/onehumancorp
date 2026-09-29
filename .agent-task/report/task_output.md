issue_title: "Implement Invoice-Grade Metering and Usage Accounting (F05)"
issue_description: |
  # Mission Queue Protocol: Implement Invoice-Grade Metering (F05)

  ## Title
  Implement Invoice-Grade Metering and Usage Accounting (F05)

  ## Problem Statement
  The current telemetry and cost reports are not invoice-grade. We need to capture durable, idempotent usage events with tenant/project/task/attempt and provider request IDs. We also need proper payer, auth mode, and rate attribution, along with reconciliation against actual provider invoices. We need to distinguish between customer-paid BYOK (Bring Your Own Key) inference and OHC-funded inference.

  ## Research Report
  - **Observed Gap:** The system currently tracks events, but does not provide an invoice-grade meter. F05 remains blocked.
  - **Requirements:**
    - Durable, idempotent usage events.
    - Attribution by payer, auth mode, provider, and model.
    - Subunit precision for costs.
    - Hard budget reservations before starting new paid work.
    - Settlement, release, replay, and restart concurrency checks.
    - Segregation of OHC inference costs from BYOK usage.
  - **Skill Provenance:** `superpowers:brainstorming` and `superpowers:using-superpowers` from revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`

  ## Design Doc
  - **Architecture Diagram:** A new metering bounded context that intercepts all provider calls to emit idempotent usage events.
  - **Mobile UX Flow:** Mobile dashboards displaying exact cost totals (with subunits) and clear separation of BYOK usage.
  - **AI Agent Integration:** Agents check hard budget limits before executing.
  - **Key Decisions:** Use integer subunits for money. Rely on database idempotency keys (e.g. attempt ID + provider ID) to prevent double counting.

  ## Implementation Prompt
  Implement an invoice-grade metering system. Create a durable event log for all model/tool usage. Ensure that each event captures `tenant_id`, `payer_type` (OHC vs BYOK), `provider`, `model`, `tokens/units`, and a unique idempotency key. Update the agent runtime to reserve budget before execution and settle afterward.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Strategy Admission
  - **Target ID:** OHC-10
  - **Selected Customer/Stage:** All supported personas, Run stage.
  - **Evidence Level:** Documented code gap (F05) in `docs/research/business_capability_and_usage_economics_audit.md`.
  - **Baseline/Result Metric:** Zero duplicate charges; 100% reconciliation with provider invoices.
  - **Dependencies/Reuse:** Existing reservation logic.
  - **Non-Goals:** Do not build a generic ERP.
  - **Authority Class:** System-level billing authority.
  - **Cost Plan:** Minimal overhead per event.
  - **Acceptance Checks:**
    - Idempotent processing verified via concurrent load tests.
    - Separation of BYOK funds confirmed in database queries.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
