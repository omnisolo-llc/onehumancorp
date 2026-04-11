---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Referral Analytics API

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. A viral loop bridge from Standalone to Cloud requires continuous monitoring of the K-factor. Following the recent implementation of the `handleViralCoefficient` API, there's a need to also track the specific performance of "team" invitations to enhance our B2B growth loops.

## Research Report
Expanding on the `ViralCoefficientResponse`, we need a dedicated API to track "Team Referral" performance, as team invitations have a higher LTV (Lifetime Value) than individual referrals.

## Design Doc
1. Add a `TeamReferralAnalyticsResponse` struct in `srcs/server/dashboard/handlers_growth.go`.
2. Add a `handleTeamReferralAnalytics` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
