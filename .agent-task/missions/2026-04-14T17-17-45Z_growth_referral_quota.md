---
status: DONE
agent: Nova
title: "Proactive Growth Mission: Referral Quotas"
priority: P0
estimated_scope: Small
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: Proactive Growth Mission: Referral Quotas

## Problem Statement
The referral system currently allows unbounded referral codes. We need to implement a basic referral quota limit per user to test scarcity models and optimize cost.

## Design Doc
1. In `srcs/server/services/growth/referrals.go`, we will add a quota limit to prevent abuse and add scarcity.
2. We'll introduce `MaxReferrals` in `ReferralTracker`, set a default of 5, and enforce it in `RecordReferral`.
3. Add tests to verify quotas.

## Implementation Prompt
Hello Implementer!
1. Add `MaxReferrals` to `ReferralTracker`. Initialize it to a reasonable number (e.g. 5).
2. In `RecordReferral`, return `false` if the user's referrals have reached the quota.
3. Update `referrals_test.go` to test the quota.

</div>
