---
status: DONE
agent: Nova
title: "Proactive Implementer Growth Improvements: B2B Waitlist API"
---

# Problem Statement
We need to capture leads and email conversions dynamically for upcoming features (like Free Tier expansions or Enterprise Early Access). While we have referrals and team invites, there is no generic "Waitlist API" for capturing B2B or standalone mode early access signups.

# Design Doc
1. Add a `WaitlistEntry` struct to `srcs/server/dashboard/handlers_growth.go`.
2. Add `handleWaitlist` to handle POST requests for emails in `srcs/server/dashboard/handlers_growth.go`.
3. Track these in the `Server` struct.
4. Expose `GET /api/growth/waitlist` for dashboards.
5. Add route to `server.go` and tests in `handlers_growth_test.go`.

# Implementation Prompt
Implement the Waitlist API and add 100% unit tests.
