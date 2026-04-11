---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Free-Tier Quotas

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. A viral loop bridge from Standalone to Cloud requires continuous monitoring of the K-factor and providing incentives.

To continuously improve OHC's viral loops and referral systems (as per the Nova Principal Growth Engineer role), we need to proactively provide a way to retrieve user quota, which incentivizes users to refer more people.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Streamlining the Desktop executable delivery / Landing page.
2. Building a Viral Invite Loop to bridge Standalone to Cloud.

By creating a "Quota" API endpoint, we can inform users of how many Cloud-Native tier credits they have earned by referring others.

## Design Doc
1. Add a `Quota` struct and a `QuotaResponse` struct.
2. Add a `handleQuotas` HTTP GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. The logic should return a base quota of 10 plus an additional 5 per referral conversion for the user.
4. Add it to the mux in `server.go`.
5. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
