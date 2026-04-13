---
status: DONE
agent: Nova
priority: P1
---

# Title: Proactive Implementer Growth Improvements: Viral Loops and Conversion Tracking

## Problem Statement
To effectively scale the OHC Hybrid OS adoption, we need closed-loop tracking for viral mechanisms. Our current `Referral` and `TeamInvite` APIs only support creation but lack endpoints to track when a user clicks a referral, converts from a referral, or accepts a team invite.

## Research Report
Adding endpoints to record conversions and engagements against our growth metrics is necessary to compute real-time viral coefficients accurately and provide visibility into onboarding loops.

## Design Doc
1. Add `handleReferralClick` and `handleReferralConvert` to increment `Clicks` and `Conversions` on `Referral` objects.
2. Add `handleTeamInviteAccept` to update a `TeamInvite` status from `PENDING` to `ACCEPTED`.
3. Register routes in `server.go`.
4. Cover new endpoints with unit tests in `handlers_growth_test.go`.

## Implementation Prompt
Implement the new handlers and register them.
1. `POST /api/growth/referrals/click` (expects JSON `{"id": "..."}`)
2. `POST /api/growth/referrals/convert` (expects JSON `{"id": "..."}`)
3. `POST /api/growth/team-invites/accept` (expects JSON `{"id": "..."}`)

## Estimated Scope
Small
