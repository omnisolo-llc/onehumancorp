issue_title: "Blocked no-work finding: evaluate resource-based charging/customer-funded inference"
issue_description: |
  The owner requested to evaluate resource-based charging/customer-funded inference and address F05 (telemetry not invoice-grade). However, the existing infrastructure does not provide real workload cost accounting, measured deployment cost, representative workload distribution, or reconciled provider invoice. Building a generic ERP without this data contradicts the current evidence-first mandate.

  - We have reviewed the current `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md`.
  - The audit clearly points out that F05 requires "Platform compute/idle/support allocation, provider-invoice reconciliation and payment collection are not completed by this ledger."
  - Further, the audit instructs: "evaluate resource-based charging/customer-funded inference. Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research. Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection."
  - Building this architecture requires the missing owner workflow evidence and actual metrics before new code is warranted, marking this a blocked task as we cannot manufacture these prerequisites.

  **Strategy Admission:**
  - **Target ID:** OHC-09/10 / F05
  - **Selected Customer/Stage:** All service business types, post-activation
  - **Evidence Level:** Blocked - missing representative costs or owner outcomes
  - **Baseline/Result Metric:** None currently
  - **Dependencies/Reuse:** Wait for owner/provider evidence
  - **Non-goals:** Building an ERP
  - **Authority Class:** Planning
  - **Cost Plan:** $0
issue_priority: P1
issue_category: Research
issue_type: Blocked No-Work Finding
issue_label: blocked
assignees: []

**Test execution constraints:**
- Intermittent concurrency failures in `server_harness` provider facade tests resolve when run single-threaded.
- Verified test command: `cargo test -p server_harness --test provider_facade -- --test-threads=1`
