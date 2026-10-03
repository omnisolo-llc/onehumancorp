outcome: blocked
issue_title: "[Research] Autonomous Field Service Estimator & Routing"
issue_description: |
  **Workflow & Issue Definition:**
  Customer: Carlos the Handyman (Service/Repair/Field Work).
  Stable Target: OHC-11 (Second business pack: adjacent consulting/tutoring or field services).
  Issue Request: Implement dynamic scheduling, autonomous field estimator (Sales Agent), routing (Operations Agent), and specific mobile UI views.

  **Evidence & Current Audit Analysis:**
  The current repository is under explicit instructions strictly prioritizing correctness, resolving existing defects, and enforcing the existing evidence/audit gates (Revision 2026-09-18-usage-audit) before starting new product features. The instructions mandate reviewing RESEARCH.md and docs/research/business_capability_and_usage_economics_audit.md.

  According to docs/research/business_capability_and_usage_economics_audit.md (Section 8 "Evidence needed before a concrete implementation plan" and "Implementation Blockers"):
  - Evaluation of managed API vs API-key/cloud-account billing is blocked pending real usage data and owner interviews.
  - Evaluation of provider-permitted native-client subscription vs local inference is blocked due to missing provider access prerequisites.
  - Furthermore, RESEARCH.md states: "First digital-service loop meets paid retention, reliability and economics gates" before proceeding to OHC-11 (Second business pack: adjacent consulting/tutoring or field services; commerce later).

  The requested work involves new vertical expansions (field service estimator, routing agents), which are explicitly deferred pending evidence-backed decisions and completion of the core digital-service loop.

  **Conclusion:**
  Implementing new epics for Field Services (OHC-11) is blocked. Explicit expansion approval and core loop verification (OHC-01 through OHC-08) is required first.

  **Loaded skills:**
  - using-superpowers (provenance: skills/using-superpowers/SKILL.md)
  - Upstream revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
