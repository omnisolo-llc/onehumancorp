issue_title: "Documentation Audit: Standing Authority and Exceptions"
issue_priority: "P1"
issue_category: "documentation"
issue_type: "audit"
issue_label: "ohc:documentation"
assignees: []
issue_description: |
  # Documentation Audit: Standing Authority, Evidence, Cost, Exceptions, and Recovery

  ## Context
  This audit assesses the current documentation state for OHC's product concerning standing authority, evidence, cost, exceptions, and recovery in alignment with the revision 2026-09-18-usage-audit. The documentation must explain how an owner accomplishes verified business work with the current product in plain language.

  ## Audit Findings

  ### 1. Standing Authority
  - **Gap**: There is a lack of clear plain-language documentation explaining what routine work owners delegate through explicit standing authority, and how new commitments outside it require approval.
  - **Requirement**: Documentation needs to outline the tenant/client scoping, owner standing authority, and how fresh approval for out-of-policy commitments is handled.

  ### 2. Evidence
  - **Gap**: The documentation does not sufficiently distinguish supported behavior from planned or historical features based on real evidence.
  - **Requirement**: Must explicitly define OHC's tracking of full client-to-cash cycles separately from individual steps and clarify that a PR count or token volume is not a business outcome.

  ### 3. Cost
  - **Gap**: Need documentation for actual cost accounting. The former invented monthly budgets are suspended.
  - **Requirement**: Need plain language explanations of estimated task cost and maximum authorized spend. Explain the itemized OHC bill, separate customer-direct provider usage (BYOK), and clear unknown or pending reconciliation.

  ### 4. Exceptions and Recovery
  - **Gap**: Insufficient guidance on handling durable exceptions, restart recovery, missing input, declined payment, stale approval, revoked credentials, and exhausted budgets.
  - **Requirement**: Need step-by-step documentation on recovery paths. Unknown provider outcomes require reconciliation before retry.

  ## Proposed Remediation
  The documentation infrastructure (Help Center, Tooltips, Walkthroughs) should be updated to address these specific gaps, prioritizing plain language and an owner/operator perspective. Future implementation tasks will focus on building out this documentation content.
