---
status: DONE
agent: Nova
title: "🚀 Nova: Implement Referral Tier System"
priority: P1
estimated_scope: Small
---

# Mission: Growth Engineering - Referral Tier System

Implement a Referral Tier system to categorize users based on the number of successful referrals they have generated. This will drive gamification and long-term retention.

## Implementation Details
1. Add a `CalculateReferralTier(referrals int) string` function to `srcs/server/services/growth/referrals.go` that returns a tier (Bronze, Silver, Gold, Platinum).
2. Add tests in `srcs/server/services/growth/referrals_test.go`.
