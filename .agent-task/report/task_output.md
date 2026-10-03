outcome: no_work
issue_title: "Implement Agentic AI Unified Inbox & Action Feed for SMB Owners"
issue_description: |
  The task requests implementing the Agentic AI Unified Inbox & Action Feed for SMB Owners (GitHub Issue #34797).
  However, based on the OneHumanCorp operating contract — revision 2026-09-18-usage-audit and the current RESEARCH.md, the implementation is explicitly blocked by missing authorization and evidence prerequisites.

  **Evidence and Gap Analysis:**
  *   **Required Authorization:** The contract strictly mandates: "New epics need an explicit evidence-backed decision" and "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  *   **Missing Evidence:** The RESEARCH.md file (Revision: 2026-09-18-usage-audit) explicitly states under "Active business-capability map" that "Implementation acceptance uses make lint and make test... Research evidence alone does not certify these gates or a shipped business outcome." Furthermore, "Existing OHC-01–12 identifiers remain cross-references; F01–F15 are audit/remediation references. Neither list is a substitute for a current GitHub issue with reproduced evidence, a bounded scope and acceptance criteria. A research session may return no new work without creating a duplicate issue."
  *   **No Active Scope Authorization:** The issue describes a massive new feature (AI Unified Inbox) involving new frontend orchestration and webhook pipelines. The current contract scope is restricted to: "Correctness before expansion: recheck F01–F15 against the current code and remediation evidence. Prioritize unresolved false payment/completion claims, usage feedback/double accounting, tenant leakage, authority and budget defects." The implementation of the Unified Inbox is not authorized without prior completion of these correctness priorities and explicit expansion gate approval in RESEARCH.md.
  *   **Superseded Strategy:** The issue references the "solo web/design/marketing services" persona (Nora) and other specific segments. The current RESEARCH.md explicitly states: "The earlier $99 subscription, 300-step allowance, $299 setup, fixed cohort/margin targets and exclusive web/design/marketing segment are suspended hypotheses, not accepted requirements... Do not turn them into... automatically dispatched features."

  **Workflow Compliance:**
  *   The required Superpowers workflow was initiated.
  *   Skill paths loaded during this session:
      *   skills/using-superpowers/SKILL.md
  *   Skill Provenance (Upstream Git Hash): 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  **Conclusion:**
  Because the requested feature represents a massive expansion outside the currently authorized bounds of defect remediation (F01-F15), relies on suspended hypotheses (specific personas), and lacks explicit expansion gate approval in RESEARCH.md, no work can be performed. The task is returned as no_work.
