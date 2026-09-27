issue_title: '✍️ Scribe: [blocked no-work finding: documentation implementation blocked due to scope override]'
issue_description: |
  **Title**: Scribe: Blocked No-Work Finding for In-App Help Center Documentation

  **Problem Statement**:
  The current task targets implementing the "In-App Help Center" features as outlined in the legacy `docs/superpowers/specs/2024-07-05-in-app-help-center-design.md` and role description. However, `RESEARCH.md` revision `2026-09-18-usage-audit` explicitly overrides older role-body examples and historical targets, stating: "This mission overrides older role-body examples below. Select one existing issue or research uncertainty; do not reopen repaired findings or treat historical pricing/segment targets as requirements."

  The instructions command to "prioritize evidence and assigned reproduced correctness defects" and explicitly forbids "random framework upgrades, forced feature generation, cosmetic churn or requirement to produce a diff every run." Furthermore, the rules state "A no-work/blocked result with evidence is valid. P0 is an actual money/data/security incident or release-blocking defect; a desirable feature is not P0."

  **Research Report**:
  Review of the `RESEARCH.md` and `docs/research/native_migration_and_remediation.md` confirms that documentation epics like the "In-App Help Center" from superseded strategic plans are not approved for implementation without explicit evidence-backed justification in the current audit context. The documentation instructions in the role description are legacy targets suspended by the current scope gate. Therefore, implementing the In-App Help Center is blocked as a no-work finding under the current authoritative guidelines.

  **Design Doc**: Not applicable (Blocked no-work finding)

  **Implementation Prompt**: Not applicable (Blocked no-work finding)

  **Priority**: P3 (Blocked)

  **Estimated Scope**: None (Blocked)

issue_priority: 'P3'
issue_category: 'documentation'
issue_type: 'research'
issue_label: 'agent-report'
assignees: []
