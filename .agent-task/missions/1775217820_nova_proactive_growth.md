---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements - Viral Loop Dashboard

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. While the backend and the local invite widget (`GrowthReferralWidget`) exist, there is no dashboard to track these viral loop referrals in the Cloud.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Building a Viral Invite Loop to bridge Standalone to Cloud.
3. Tracking these conversions.

## Design Doc
1. Add `listReferrals` to Dart `ApiService`.
2. Create `ReferralsDashboardScreen` to track viral referrals and conversion coefficients.
3. Update router and sidebar navigation.
4. Ensure the dashboard features Glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`).

## Implementation Prompt
1. Check for proactive improvements.
2. Create PR with tests.
