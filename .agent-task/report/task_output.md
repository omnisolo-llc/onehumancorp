outcome: blocked
issue_title: GitHub Issue #31649: Assistant-First Work Feed & Triage System for Owners
issue_description: |
  **No-Work / Blocked Outcome**

  After reviewing the repository and research documents, specifically the `RESEARCH.md` update (revision: 2026-09-18-usage-audit) and `docs/research/business_capability_and_usage_economics_audit.md`, the requested "Assistant-First Work Feed & Triage System for Owners" is blocked from implementation.

  The current operational instructions state: "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue... Do not start new pricing, segment or feature epics from the superseded strategy. A proposed change needs a current-code inventory, observed owner need and reuse analysis."

  Furthermore, the requirement to "implement the `AssistantShell` and `PriorityFeed` components in the `src/ui/tauri` app... Focus on the Apple/Ubiquiti translucent design system" constitutes a major UI rewrite and new epic, which explicitly conflicts with the current product scope guidelines:
  - "Preserve existing useful screens rather than rewriting the whole application."
  - "Defer... generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome."
  - "If current acceptance criteria are already satisfied, return a justified no-work result instead of mandatory restyling, state-management refactoring or extra dashboard panels."

  A `UnifiedFeed` (`src/ui/next/src/app/unified-feed/page.tsx`) and `TriagePage` (`src/ui/next/src/app/triage/page.tsx`) already exist and manage queue items, fulfilling similar goals to what is proposed. Replacing or duplicating these without explicit, new validation data contradicts the direct command to "Integrate existing tools instead of rebuilding them" and the evidence-first principle.

  Therefore, the expansion is gated. A no-work / blocked finding is submitted to retain existing implementations without introducing untested rewrites or non-approved visual orchestrations.
issue_priority: P1
issue_category: UI
issue_type: Feature Request
issue_label: blocked
assignees: []
