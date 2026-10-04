outcome: no_work
issue_title: "GitHub Issue #31967: Implement 'Zero-Click' Autonomous Onboarding Agent to Solve SMB Setup Paralysis"
issue_description: |
  The requested "Zero-Click Onboarding Agent" feature is already implemented and verified in the codebase.

  Verification evidence:
  1. The core conversational setup UI is fully implemented in `src/ui/next/src/app/onboarding/zero-click/page.tsx` and `src/ui/next/src/app/onboarding/zero-click/components/OnboardingChatAgent.tsx`.
  2. The mobile-first layout (375px) is already built, handling the conversational flow with KAIROS without requiring settings or dashboard navigation.
  3. The required 5 Playwright E2E tests are already present and cover the end-to-end flow in `src/e2e/tests/zero_click_onboarding.spec.ts`:
     - 'User completes chat onboarding and sees welcome card on feed'
     - 'Conversational Setup prevents empty submissions'
     - 'Conversational Setup opens image upload input when toggled'
     - 'Conversational Setup maintains history after reload'
     - 'Conversational Setup renders user messages correctly'

  Since the acceptance criteria and requested implementation are completely satisfied by the existing code, no further modifications are required.
