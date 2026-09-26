issue_title: '🗺️ Guide: No-Work/Blocked Onboarding Optimization'
issue_description: |
  # Superpowers Workflow Provenance
  - Loaded skills: brainstorming, systematic-debugging, using-superpowers
  - Upstream repo: https://github.com/obra/superpowers
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks performed: Code review of `src/ui/next/src/app/onboarding`, `src/e2e/onboarding.mock-contract.ts`, and `src/server/api/onboarding/mod.rs`. Test execution via `npm run test:e2e`.
  - Outcomes: The current onboarding flow (Zero-Click / Instant Build) accurately reflects the state requirements, handles cross-device syncs natively via `syncStateToBackend`, limits inputs strictly per backend parsing (`statePayload.ts`), and successfully binds to tenant boundaries without any observed capability gaps or denied permission blocking issues. As instructed ("If no justified current-scope gap exists, return a no-work finding instead of forced visual refactoring."), no functional codebase modifications are required.

  # Findings
  1. The onboarding process successfully serializes and synchronizes `step`, `chatStep`, and business details between client (`store.ts`) and backend (`/api/v1/onboarding/state`).
  2. Input length constraints and boundary conditions are properly enforced both in the UI (`hasAtMostChars` in Next.js payload mapper) and on the backend (`validate_chat_request` in `api/onboarding/mod.rs`).
  3. No explicit missing dependencies or capability gaps were observed in the established `F01-F15` audit ledger that pertain directly to onboarding friction blockers without overextending into the "unsupported integrations" domain.
issue_priority: 'P2'
issue_category: 'growth'
issue_type: 'research'
issue_label: 'agent-report'
assignees: []
