issue_title: "Implement invoice-grade telemetry and cost reporting (F05)"
issue_description: |
  **Title**: Implement invoice-grade telemetry and cost reporting (F05)

  **Problem Statement**:
  Currently, telemetry and cost reports are not at the level of an invoice-grade meter. We need durable, idempotent usage tracking with proper payer, auth, and rate attribution to ensure reliable business operations and avoid duplicate BYOK debits. This impacts the ability to reliably charge customers and track costs accurately.

  **Research Report**:
  Based on the active business-capability map and scope priorities, a core necessity for the OHC subscription model is accurate tracking of serving costs and usage.
  Existing metrics do not support integer subunits or tenant-specific reads effectively for billing purposes.
  Proper reconciliation requires an authoritative ledger of usage.

  **Design Doc**:
  - Architecture diagram (Mermaid.js):
    ```mermaid
    graph TD
      A[API/Tool Request] --> B[Idempotency Layer]
      B --> C[Usage Metering Service]
      C --> D[(Usage Database - Integer Subunits)]
      D --> E[Reconciliation Job]
    ```
  - UI wireframes or screen flow description (375px first):
    - A simple usage dashboard card showing current month costs vs budget.
  - Mobile UX flow:
    - User opens the app, taps on 'Settings' -> 'Usage & Billing'.
    - Views a clear breakdown of AI operations cost without complex developer terminology.
  - AI agent integration points:
    - Agents report usage before and after execution to the metering service.
  - Key design decisions and why:
    - Store amounts in integer subunits to prevent floating-point errors.
    - Tenant-isolated read paths for strict multi-tenancy.

  **Implementation Prompt**:
  Implement the durable usage metering service in Rust. Update the database schema to store amounts in integer subunits. Ensure that all model calls and tool executions record their usage idempotently. Add tenant-specific read endpoints for the frontend to display usage. Acceptance criteria: A tool request must generate exactly one usage record, and duplicate requests with the same idempotency key must not double-charge.

  **Priority**: P0

  **Estimated Scope**: Large

  **Strategy Admission**:
  Target ID: OHC-05
  Segment: Nora (agency principal) / service professional.
  Stage: Days 15-45 (enforcing isolation, budgets, authorization).
  Evidence level: Documented.
  Baseline/result metric: 100% of recorded tool executions have a corresponding idempotent cost record without duplicate billing.
  Dependencies/reuse: Reuse existing Postgres database and Rust backend infrastructure.
  Non-goals: Do not build a generic ERP or process actual Stripe payments in this issue.
  Authority class: Core platform infrastructure.
  Cost plan: Standard database write cost per event.
  Happy/failure-path acceptance checks:
  - Happy path: A successful agent tool call adds cost to the tenant's meter.
  - Failure path: A failed provider request does not deduct budget if it didn't incur cost, or records the error without duplicate charges on retry.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
