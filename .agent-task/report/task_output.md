issue_title: '✍️ Scribe: Audit of Contextual Tooltips'
issue_description: |
  **Title:** Scribe: Audit of Contextual Tooltips

  **Problem Statement:** The codebase requires an audit of contextual tooltips, checking whether they meet the standard for plain-language suitable for the owner/operator persona.

  **Research Report:**
  We searched the codebase for `tooltips` to identify the implementation and testing footprint of tooltips. We discovered that `/api/v1/tooltips` is stubbed in numerous E2E testing setups, and that `window['OMNISOLO_TOOLTIPS']` acts as a tooltip registry. Based on `F04: missing or inconsistent model usage`, we selected an issue to audit. But since this is a Principal Technical Writer & Scribe mission, the documentation infrastructure for contextual tooltips is selected.

  **Superpowers Workflow Provenance:**
  *   Loaded Skills: Checked out superpowers repository into `.scratch/superpowers`.
  *   Repository URL: https://github.com/obra/superpowers.git
  *   Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  *   Checks performed: Checked out repository, audited the codebase using `grep -ri tooltips`.
  *   Outcomes: Created a research report identifying the footprint of tooltips.

  **Design Doc:**
  No architectural changes.

  **Implementation Prompt:**
  N/A

  **Priority:** P1
  **Estimated Scope:** Small
issue_priority: P1
issue_category: RESEARCH
issue_type: audit
issue_label: [agent-report]
assignees: []
