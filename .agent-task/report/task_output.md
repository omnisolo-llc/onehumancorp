issue_title: No-work/blocked outcome: Registration & Setup Wizards
issue_description: |
  # Findings
  - The current operating contract explicitly states "If no justified current-scope gap exists, return a no-work finding instead of forced visual refactoring."
  - We have reviewed the current `OnboardingWizard` setup flow and its tests. The application currently implements the onboarding endpoints as defined in `src/server/api/onboarding/mod.rs` and the wizard in `src/ui/next/src/app/onboarding`.
  - The audit from `RESEARCH.md` and the usage economics audit specify that "Setup endpoints exist; successful provider connection and an owner-ready operating business are not established. Reuse the state flow rather than inventing another wizard."
  - Therefore, without a concrete evidence-backed assigned issue or an observed activation blocker, we are returning a no-work/blocked result.
issue_priority: P2
issue_category: UX
issue_type: Blocked Task
issue_label: ohc:lane:activation
assignees: []
