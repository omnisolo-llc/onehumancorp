---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Referral Conversion Tracking

## Problem Statement
The growth strategy audit emphasizes building a Viral Invite Loop to bridge Standalone to Cloud. Currently, we can create referrals via `POST /api/growth/referrals` and track the overall Viral Coefficient, but we lack specific endpoints to track when a referral link is clicked or when a user actually converts (signs up/joins a team).

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Building a Viral Invite Loop to bridge Standalone to Cloud.

By adding specific tracking endpoints for clicks and conversions, we can measure the exact conversion rate of our Sovereign-to-Cloud referral loop and feed accurate data into the K-factor calculation.

## Design Doc
1. Add `handleReferralClick` (increments click count for a referral ID) in `srcs/server/dashboard/handlers_growth.go`.
2. Add `handleReferralConvert` (increments conversion count for a referral ID) in `srcs/server/dashboard/handlers_growth.go`.
3. Add these routes to `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API endpoints for tracking clicks and conversions.
2. Ensure tests pass.
