issue_title: "💰 Miser: F05 Telemetry is not an invoice-grade meter"
issue_description: |
  **Issue/Target ID**: F05 (from docs/research/native_migration_and_remediation.md)

  **Observed versus Inferred Gap**: The current code implements telemetry and cost reports, but these are not invoice-grade meters. F05 remains listed as "Open" in the audit ledger. The cost measurements currently available are lacking robust payer, auth, rate attribution, integer subunits, tenant-level read separation, reconciliation logic, and preventing duplicate BYOK (Bring Your Own Key) debits.

  **Expected Result**: This is an explicitly identified research/audit assignment from `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md` regarding missing elements for robust cost tracking. The current task objective per the instruction is to act as **Miser (L7)** and address cost engineering or evaluate cost structures, or provide an evidence-backed "no-work/blocked" result if we lack prerequisites like actual live data or owner interviews.

  **Dependencies/Blockers**: The problem explicitly requires actual production evidence, reconciliation of real provider invoices, and owner interviews to select compute/API prices or allowances (as documented in Section 8 of the economics audit). Because we do not have actual provider invoices, real owner cost data, or authorization to integrate live payment gateways with live customer money right now, we cannot complete a live "invoice-grade meter" that reconciles real money. As the prompt instructs, "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success." Therefore, we are logging a no-work/blocked result for F05 because it requires real-world prerequisites (invoice reconciliation, owner interviews, provider access bounds) that are not present in this sandbox environment.

  **Conclusion**: Blocked. F05 requires real provider invoices and owner interviews to formulate correct pricing models and test exact reconciliation. Cannot implement a mock invoice meter without real evidence.
issue_priority: "P0"
issue_category: "finance"
issue_type: "research"
issue_label: "ohc:lane:finance"
assignees: []
