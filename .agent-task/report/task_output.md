issue_title: '🧙 Wizard: Evaluate Setup Process & Optimizations'
issue_description: |
  **Title**: 🧙 Wizard: Evaluate Setup Process & Optimizations

  **Problem Statement**: The current `business-setup` onboarding wizard requires evaluation against the new OHC Premium Design Standards (Apple / Ubiquiti style translucent glass, specific spacing, etc.). We need to verify if the feature exists, map its state, and determine if it meets the new requirements.

  **Research Report**:
  - Explored the codebase, including `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`.
  - Discovered `src/ui/next/src/app/business-setup/page.tsx`, which contains a basic entry point redirecting to `/onboarding` for business setup.
  - Investigated various other `wizard` and `setup` pages across the Next.js UI.
  - The business setup onboarding flow does not appear to be a fully featured multi-step wizard in its current form in `business-setup`, but rather a landing page that links to `/onboarding`.
  - The current state of `business-setup` does not violate the design guidelines outright as it's a simple landing page, but it doesn't implement a complex state machine wizard either.
  - A comprehensive overhaul of the onboarding flow to meet the new Premium Design Standards across the entire `/onboarding` process is outside the scope of a single "no-work/blocked" finding implementation, but the entry point itself is functional.
  - No new capabilities or verticals were added, adhering to the expansion gate in `RESEARCH.md`.

  **Design Doc**:
  - The entry page at `src/ui/next/src/app/business-setup/page.tsx` uses standard Tailwind classes and an `<AppShell>`.
  - To fully implement the Apple/Ubiquiti glass style, significant CSS token updates and refactoring of the global UI components would be required.

  **Implementation Prompt**: None. The current entry point is functional and no specific bug or new feature request was provided in the prompt beyond auditing and optimizing the wizard experience. The flow currently redirects to the main onboarding path.

  **Priority**: P2

  **Estimated Scope**: Small
issue_priority: 'P2'
issue_category: 'ux'
issue_type: 'task'
issue_label: 'agent-report'
assignees: []
