issue_title: "✍️ Scribe: Investigate Documentation Evidence for In-App Help Center and Tooltips"
issue_description: |
  **Title:** Investigate Documentation Evidence for In-App Help Center and Tooltips

  **Problem Statement:** Following the current active capability map, we need to document how an owner accomplishes verified business work with the current product. It's unclear whether an In-App Help Center and Contextual Tooltips are supported or if they were merely planned features from superseded strategy. We need an evidence-backed decision on whether they exist in source, are mounted, tested, or verified before building new documentation infrastructure.

  **Research Report:**
  We investigated the current code and audit evidence according to revision 2026-09-18-usage-audit:
  1. The audit ledger (F01-F15) and existing capability gaps do not explicitly mention the absence or existence of an In-App Help Center or Contextual Tooltip API.
  2. Legacy Help and Voice Scripts were evaluated as part of the migration acceptance register, with safe DOM text and validated destinations utilized instead of interpolated HTML (baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20`, branch `fix/bazel-modernization-and-cleanup`). Help messages and article titles remain inert text, and links reject insecure URLs.
  3. No concrete provider-sandbox or real-owner verification exists for a functional Contextual Tooltip API integrated with a React frontend. The instructions from superseded strategy defining a "floating 'Ask anything' chat button" or "interactive API reference" lack current-code evidence.
  4. There are no existing issues reporting this as a blocked user workflow.

  Evidence of superpowers workflow:
  Loaded superpowers skill: using-superpowers, brainstorming
  Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71

  **Source Dates:** Baseline 2026-09-18. Source baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20` on `fix/bazel-modernization-and-cleanup`.

  **Study Populations:** Evaluated for non-technical service, retail, and physical-operation workflows.

  **Uncertainties:** It is uncertain which specific topics have the highest drop-off rate or if owner effort is significantly reduced by built-in tooltips compared to existing manual workflows.

  **Metric Definitions:**
  - Reduced support tickets about "how do I do X?".
  - Improved adoption metric, measured by number of help articles accessed divided by active user count.
  - Number of workflow steps completed without external help.

  **Design Doc:**
  N/A - Research only. No implementation is proposed because the prerequisite code verification and owner evidence do not justify building new documentation capability outside of resolving concrete defects.

  **Implementation Prompt:**
  N/A - Research only. A no-work result is reported due to a lack of evidence-backed need in the current scope priority.

  **Priority:** P3

  **Estimated Scope:** Small (Research only)

issue_priority: "P3"
issue_category: "documentation"
issue_type: "research"
issue_label: "documentation"
assignees: []
