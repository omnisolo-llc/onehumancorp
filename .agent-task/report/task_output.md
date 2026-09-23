issue_title: "Implement F05: Invoice-grade telemetry and cost accounting"
issue_description: |
  # Mission Queue Protocol Brief

  ## Title
  Implement F05: Invoice-grade telemetry and cost accounting

  ## Problem Statement
  Currently, cost telemetry in the system does not meet invoice-grade standards. The system uses floating-point values instead of integer sub-units, does not durably deduplicate records, and lacks precise payer, authentication, and rate attribution. If left as is, it risks double-billing BYOK (Bring Your Own Key) customers and dropping records, which undermines owner trust and leads to revenue leakage or billing disputes.

  ## Research Report
  The audit `docs/research/native_migration_and_remediation.md` and `docs/research/business_capability_and_usage_economics_audit.md` identify F05 as an open gap. The `auditor.rs` and `service.rs` in `src/server/services/billing/` use `f64` for cost tracking and lack a durable structure that guarantees idempotency and attributes usage properly (e.g. distinguishing between managed API and customer API key to avoid double-charging BYOK). This requires moving to a ledger model using integer subunits (cents/micro-cents) for financial precision, attributing the payer, and handling idempotent updates.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    flowchart TD
      API[Billing Service API] --> |track_token_usage| Ledger[Cost Ledger / Auditor]
      Ledger --> |validate & deduplicate| Store[(Idempotent DB/Store)]
      Ledger --> |aggregate| Snapshot[Tenant/Agent Snapshot]
      Store --> |reconciliation| Invoice[Billing Invoice Engine]
    ```
  - **UI Wireframes**: 375px mobile view of the "Usage & Billing" screen showing an itemized list of OHC-funded versus customer-funded (BYOK) usage, using Translucent Glass and UniFi modular cards.
  - **Mobile UX Flow**: The user opens the Billing tab, sees a clear distinction between "Managed Compute" and "Direct Provider API (BYOK)" usage, and views exact cent-accurate totals without technical jargon.
  - **AI Agent Integration Points**: Agents emitting token usage will include an idempotency key and authentication context (payer mode). The ledger will deduplicate based on this key.
  - **Key Design Decisions**:
    - Use `i64` for micro-cents or integer cents to avoid floating point errors.
    - Introduce an `idempotency_key` and `payer_mode` (e.g., `Managed`, `BYOK`) to the `TokenUsage` struct and `AuditEvent`.
    - Do not charge BYOK usage against the tenant's OHC managed balance.

  ## Implementation Prompt
  **To the Implementer**: Update the `TokenUsage` protobuf and `AuditEvent` struct to include `idempotency_key` and `payer_mode`. Refactor `CostAuditor` to use `i64` for financial tracking (e.g., micro-cents) instead of `f64`. Ensure that when `payer_mode` indicates a BYOK (customer-paid) key, the usage is recorded for visibility but is NOT added to the OHC billable total. Implement deduplication based on the `idempotency_key`. Add comprehensive unit tests verifying that BYOK usage is not double-charged and that duplicate events are ignored. Ensure all changes satisfy F05 in `docs/research/native_migration_and_remediation.md`.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - Target ID: F05 (Telemetry/Cost reports)
  - Selected customer/stage: Launch/Run stage
  - Evidence level: Documented
  - Baseline/Result metric: 100% of telemetry events properly attributed and deduplicated.
  - Dependencies/reuse: Reuses existing `CostAuditor` and `BillingService`.
  - Non-goals: Not implementing actual payment collection or Stripe integration in this step.
  - Authority class: Platform telemetry
  - Cost plan: Negligible compute overhead for deduplication.
  - Acceptance checks: `make test-backend` passes, and the F05 regression tests confirm no double BYOK billing and no floating-point truncation issues.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
