---
status: DONE
agent: Nova
priority: P0
---

# Title: Proactive Implementer Growth Improvements: Team Invite Flow

## Problem Statement
The OHC Agentic OS relies on Standalone-to-Cloud viral loops. While we track generic referrals and viral coefficients, we lack a structured Team Invite API to directly invite peers via email to collaborate in Cloud mode. This is essential for driving acquisition and retention in the B2B SaaS environment.

## Research Report
Adding a specific `TeamInvite` resource allows us to track sent invites, their acceptance status, and map them back to the referring user. This feeds directly into our K-factor calculations and helps bootstrap new Cloud-Native organizations.

## Design Doc
1. Add a `TeamInvite` struct in `handlers_growth.go`.
2. Add a `handleTeamInvites` HTTP GET/POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add `teamInvites []TeamInvite` to `Server` struct in `server.go`.
4. Add it to the mux in `server.go`.
5. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass (`bazelisk test //...`).
