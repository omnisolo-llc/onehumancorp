issue_title: "F05 telemetry not invoice-grade"
issue_description: |
  **Title:** F05 telemetry not invoice-grade

  **Problem Statement:**
  The current implementation of telemetry and cost reports does not act as an invoice-grade meter. As stated in `docs/research/native_migration_and_remediation.md`, the requirement is to use "Integer micro-unit durable records bind tenant/task/attempt/provider/model/payer/rate revision; duplicate provider receipts cannot be charged twice; customer-direct usage is not debited as managed inference."
  The `src/server/telemetry.rs`, `src/server/billing.rs`, and `src/server/pricing/cost_aggregator.rs` implementations record token usage and infer cost but lack durable idempotent tracking tied to specific tasks/attempts and specific payer/auth attributions with external provider invoice reconciliation. The billing APIs expose some usage, but it does not satisfy the "invoice-grade meter" requirements.

  **Research Report:**
  The system currently implements usage and cost tracking in several places:
  1. `src/server/api/telemetry.rs` receives batched telemetry (e.g., token usage) and asynchronously records it to DB (`telemetry_buffer`) and Redis. It calculates cost instantly based on predefined internal rate lists without robust error handling on failed settlement or idempotency.
  2. `src/server/pricing/cost_aggregator.rs` rolls up costs grouping by date/metric from `telemetry_buffer`.
  3. `src/server/billing.rs` offers in-memory summaries and rate-limiting but does not maintain a strict idempotent ledger that binds to task attempts.

  However, to implement F05 correctly, we need prerequisites that are currently blocked:
  1. No definition of "payer/auth/rate attribution" payloads and where they originate from the worker execution contexts.
  2. The actual "provider invoice reconciliation" requires external data/APIs that we are explicitly forbidden from guessing or accessing live customer data for without authorization.
  3. The current telemetry architecture receives batches that lack a unique "attempt/provider request ID" required for deduplication.

  **Design Doc:**
  The planned architecture (blocked pending requirements):
  - Modify `TelemetryRow` and `telemetry_buffer` to include a unique `request_id`, `payer_id`, and `auth_mode`.
  - Migrate cost tracking to a dedicated ledger table (e.g., `usage_ledger`) that enforces idempotency on `request_id`.
  - Update `src/server/api/telemetry.rs` to ingest and persist these fields.
  - Implement a reconciliation worker that compares `usage_ledger` with provider reports.

  **Implementation Prompt:**
  Not applicable at this stage due to blocked prerequisites.

  **Priority:** P2
  **Estimated Scope:** Medium

  **Superpowers Provenance:**
  - Loaded `using-superpowers` from revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`.
  - Code search and file analysis confirmed the missing idempotency and reconciliation components.
  - Checks performed: Verified `src/server/api/telemetry.rs`, `src/server/billing.rs`, `src/server/pricing/cost_aggregator.rs`.

issue_priority: P2
issue_category: finance
issue_type: feature
issue_label: [agent-report]
assignees: []