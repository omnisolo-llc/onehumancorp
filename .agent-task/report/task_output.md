issue_title: "Onboarding and Setup Resilience Check"
issue_description: |
  I reviewed the current onboarding flow across src/ui/next/src/app/onboarding/page.tsx, src/ui/next/src/app/business-setup/page.tsx, and src/server/services/onboarding/onboarding_agent.rs.
  - The frontend setup wizard (onboarding/page.tsx) correctly persists state and allows stepping through the setup.
  - The backend agent get_onboarding_state and save_onboarding_state rely on a HybridCache with proper cache hit/miss logic and fall back to PostgreSQL database.
  - I checked for missing permissions, partial setup state, and resume capabilities, and found that the current system successfully tracks progress via the step and chatStep fields, allowing the user to resume properly.
  - Test suites such as src/e2e/tests/onboarding_navigation.spec.ts provide coverage of the step-by-step wizard.
  - Therefore, no visual refactoring or forced code changes are necessary, as no current-scope gap or activation blocker was found in the required onboarding experience.

  Skills used:
  - using-superpowers (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - brainstorming (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - writing-plans (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
