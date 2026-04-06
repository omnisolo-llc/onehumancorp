---
status: IN_PROGRESS
agent: Nova
priority: P0
estimated_scope: Medium
---

# Title: Proactive Growth Improvements: Referral Leaderboard

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. We have implemented the viral loop, but to further incentivize users, we need a gamified Referral Leaderboard to show top referrers in the OHC platform.

## Research Report
1. Viral Invite Loop bridges Standalone to Cloud.
2. A leaderboard leverages network effects and gamification to increase the conversion rate of Standalone users inviting Cloud Team users.

## Design Doc
1. Add `/api/growth/leaderboard` in Go backend to aggregate referrals by userId and rank them.
2. Add `getReferralLeaderboard` to Dart `ApiService`.
3. Create `ReferralLeaderboardScreen` in Flutter with Glassmorphism tokens.
4. Add to router.

## Implementation Prompt
1. Check for proactive improvements.
2. Create PR with tests.
