issue_title: "✍️ Scribe: [blocked no-work finding: documentation implementation task]"
issue_description: |
  **Observation:**
  The prompt instructs me to act as Principal Technical Writer & Scribe (L7) and build "In-App Help Center", "Contextual Tooltips", "Interactive Walkthroughs", "AI-Powered Help Chat", etc. using OHC premium tokens and plain language.

  However, the `RESEARCH.md` and `AGENTS.md` explicitly constrain this instruction. The final OHC scope check states:
  > Apply revision 2026-09-18-usage-audit over conflicting role-body instructions. Suspend the old fixed-price, 300-step, exclusive agency-segment and fixed margin targets. First audit the actual code and owner/provider evidence. New epics need an explicit evidence-backed decision; assigned concrete defect work may continue.

  And specifically:
  > The owner explicitly requests completing the migration and recorded defect remediation, not another research-only plan. Keep M01-M07 and F01-F15 status, exact command/test counts, and unresolved evidence in the remediation ledger.

  The prompt says: "This mission overrides older role-body examples below. Select one existing issue or research uncertainty; do not reopen repaired findings or treat historical pricing/segment targets as requirements."

  The requested documentation and onboarding UI (Help Center, Walkthroughs) are entirely new features/epics. The instructions explicitly say: "Defer new viral generators... generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome. Retain useful existing implementations; avoid deletion or a platform rewrite just to fit this document."

  Since I am tasked with working on the documentation role but am constrained by the system's "revision 2026-09-18-usage-audit" to complete the migration/recorded defect remediation instead of building new un-evidenced UI/documentation epics, the implementation of these help features is a blocked no-work finding.

  I will report this as a blocked no-work finding for the documentation Epic, as we must focus on the M01-M07 and F04 defect remediation.
issue_priority: "P2"
issue_category: "documentation"
issue_type: "blocked_feature"
issue_label: "ohc:lane:documentation"
assignees: []
