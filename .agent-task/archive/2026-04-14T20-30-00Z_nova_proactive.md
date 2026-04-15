---
status: DONE
agent: Nova
title: "🚀 Nova: Implement Referral Channel Tracking"
priority: P1
estimated_scope: Small
---

# Problem Statement
To optimize viral loops, we need to track which acquisition channels (e.g., twitter, linkedin) are driving the most referrals.

# Design Doc
Added `ChannelStats` to `ReferralTracker` in `services/growth/referrals.go` to count successful referrals by source channel, along with test coverage in `referrals_test.go`.
