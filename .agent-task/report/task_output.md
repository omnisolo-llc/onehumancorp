issue_title: "[no-work finding] Documentation Audit for Standing Authority and Account Connections"
issue_description: |
  **Research Provenance**
  - Read `docs/research/business_capability_and_usage_economics_audit.md`
  - Read `docs/research/native_migration_and_remediation.md` (F12)
  - Reviewed Help Center documentation in `docs/help_center/` and `src/server/services/docs/service.rs`.
  - Used Superpowers skill: `using-superpowers` and `brainstorming` (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d).

  **Findings**
  1. The Help Center correctly includes articles on "Setup, Connected Accounts, and Standing Authority", "How to Send Proposals and Collect Payments Securely", and "Connected Accounts and Standing Authority".
  2. These articles reflect the owner lens and use plain language (e.g., maximum 8th-grade reading level, no technical jargon), explaining how rules for sending emails/payments are set, and how to handle exceptions (like payment link failures).
  3. The `DocsService` in `src/server/services/docs/service.rs` properly serves these articles to the Next.js frontend via `/api/v1/help/`.
  4. The audit finding F12 ("Simulation, unknown provider outcome and approval paths can look like completion") is marked as "Blocked" in the remediation ledger. The documentation already accurately states: "If your payment provider disconnects and cannot make a link, the invoice will say 'Draft/Pending Provider'. Just try again later. The app will safely try again without charging anyone twice." This aligns the user expectations with the truthful states required by the new operating contract.
  5. There are no outstanding, unaddressed documentation requirements for the currently authorized implementation scope. The legacy "10-pillar" or "12-agent" documentation mandates are superseded.

  **Recommendation**
  - No new documentation features or code changes are required at this time. This is a no-work finding.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
