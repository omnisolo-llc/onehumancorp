issue_title: "[no-work finding] Evaluate Cloud Bridge Referral Loop"
issue_description: |
  # Findings

  We reviewed the current codebase to look for opportunities to implement the "Sovereign-to-Cloud Loop".
  The repository already contains `GrowthReferralWidget.tsx` which implements exactly this viral referral loop to bridge Standalone to Cloud. It also has tests for it (`src/e2e/cloud-bridge.spec.ts` and `src/e2e/viral_invite_loop.spec.ts`), as well as backend API support (`src/server/services/growth/invites.rs` and `src/server/api/growth.rs` and frontend route `/api/v1/growth/cloud-bridge/invite/route.ts`).

  Therefore, no code changes are required as the feature has already been implemented.
issue_priority: "P2"
issue_category: "Growth"
issue_type: "Research"
issue_label: "no-work finding"
assignees: []
