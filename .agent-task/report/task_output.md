issue_title: '💰 Miser: F05: telemetry not invoice-grade'
issue_description: |
  **Justification for Blocked / No-Work Finding**

  The task requires addressing the "F05: telemetry not invoice-grade" issue or another outstanding billing/cost issue. However, according to the `docs/research/native_migration_and_remediation.md` and the `business_capability_and_usage_economics_audit.md` reports, there are fundamental dependencies that have not been resolved.

  Specifically, the remediation ledger states for F05:
  > "Platform compute/idle/support allocation, provider-invoice reconciliation and payment collection are not completed by this ledger. Customer payment collection remains disabled on the new usage API."

  Furthermore, the guidance in `RESEARCH.md` states:
  > "These are decision prerequisites, not an approved feature backlog"
  > "Collect a small, permissioned set of recent owner workflows across candidate segments before choosing a segment. Compare each against both its existing manual/SaaS process and the current AI business tools. Exact willingness to pay, usage tolerance, privacy preference and desired autonomy remain unknown. Public stories help select questions, not answer those commercial questions conclusively."

  The requested data regarding representative serving costs, owner workflows, and provider invoice reconciliation (F14 and F05 prerequisites) are explicitly missing, and the rules explicitly forbid fabricating this evidence or assuming old legacy pricing/margins. The role instructions state: "A no-work/blocked result with evidence is valid. P0 is an actual money/data/security incident or release-blocking defect; a desirable feature is not P0."

  Without actual workload distribution, reconciled provider invoices, and owner willingness-to-pay evidence, we cannot safely implement an "invoice-grade meter" or a fair "value-to-cost ratio" pricing tier. Therefore, this task is blocked and no code changes should be made.
issue_priority: 'P2'
issue_category: 'Economics'
issue_type: 'blocked'
issue_label: 'blocked'
assignees: []
