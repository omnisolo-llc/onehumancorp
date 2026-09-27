issue_title: "Blocked No-Work Finding: F05 - Telemetry not invoice-grade"
issue_description: |
  **Title:** Blocked No-Work Finding: F05 - Telemetry not invoice-grade

  **Problem Statement:**
  Finding F05 states that current telemetry/cost reports are not an invoice-grade meter. It requires durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation, and no duplicate BYOK debit. It is currently blocked.

  **Research Report:**
  We investigated the current implementation in `src/server/services/billing/auditor.rs` and `src/server/billing.rs`. The `CostAuditor` currently operates entirely in-memory using `HashMap` and `Mutex`.

  - It uses `f64` for dollar amounts in some places (like `get_tenant_payment_fees`) and scales by `100.0` and rounds to `i64` for cents in others.
  - It does not have durable storage (it is an in-memory struct created via `event_pipeline`).
  - It lacks payer/auth mode, rate card revision binding, provider request IDs, and deduplication (it blindly accepts `AuditEvent` instances and updates counters).
  - The telemetry pipeline is one-way: `event_pipeline` sets up an in-memory auditor and passes events along to `telemetry_tx`.

  However, this finding is explicitly marked as **Blocked** in `docs/research/native_migration_and_remediation.md`:

  ```markdown
  | F05 | Current telemetry/cost reports are not an invoice-grade meter | Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit | Blocked |
  ```

  The audit `docs/research/business_capability_and_usage_economics_audit.md` states: "Platform compute/idle/support allocation, provider-invoice reconciliation and payment collection are not completed by this ledger. Customer payment collection remains disabled on the new usage API." and "No supported native-client subscription mode was demonstrated end-to-end in OHC."

  Because resolving F05 requires external architectural dependencies (database schema for usage, defining the exact structure for idempotent requests/payer attribution/BYOK differentiation, provider invoice reconciliation, etc.) which are not fully defined or present in the current codebase, this issue is a blocked no-work finding. We will not fabricate a partial fix.

  **Implementation Prompt:**
  None. This is a blocked finding.

  **Priority:** ""
  **Estimated Scope:** ""

issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
