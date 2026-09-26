issue_title: "✍️ Scribe: Audit of documentation and capability gaps"
issue_description: |
  # Documentation Audit and Remediation Report

  **Superpowers workflow provenance:**
  - Loaded skill: `using-superpowers`, `brainstorming`, `verification-before-completion`
  - Upstream repository: https://github.com/obra/superpowers/
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`

  ## Goal
  Select one existing issue or research uncertainty related to the documentation of current capabilities. Assess documentation features according to the `Scribe (L7)` persona rules under the 2026-09-18 usage audit bounds.

  ## Investigation
  The instruction from the user required selecting one existing issue or research uncertainty. From the `business_capability_and_usage_economics_audit.md` and the `native_migration_and_remediation.md` ledgers, I selected F14 (No measured representative serving costs or owner outcomes) and F15 (Existing commerce/fulfillment assets contradict premature exclusive segment), along with other items, to investigate for remediation.

  The instruction requires documentation features (help center, tooltips, walkthroughs) to be implemented for actual money/data/security incidents or verified business workflows *before* they can be safely promoted or enabled for automation. However, the audit notes that we currently lack these foundational metrics (F14) and are blocked due to missing business/economic data. The older broad mandates to build 10 pillars or twelve agents (or generic help centers/tooltips for suspended features) are explicitly overridden by the `2026-09-18-usage-audit`.

  ## Finding
  The instruction is **Blocked / No-Work**.

  We cannot build a new generic Help Center, interactive walkthroughs, or an AI-powered help chat because the underlying product strategy and feature availability are currently under audit, and many of the features they would document are suspended or lacking core business evidence. Documentation should reflect verifiable workflows. Currently, the documentation in `docs/help_center` correctly states the limited capabilities for supported flows without fabricating features. Therefore, expanding the documentation features per the legacy `Scribe` role constraints without resolving the foundational business gaps first would violate the current operating contract which states "New epics need an explicit evidence-backed decision" and "Do not interpret public anecdotes as customer interviews".
issue_priority: "P2"
issue_category: "documentation"
issue_type: "research"
issue_label: "ohc:lane:documentation"
assignees:
  - Automator
