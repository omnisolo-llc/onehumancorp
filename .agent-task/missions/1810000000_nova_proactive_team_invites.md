---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invites API

## Problem Statement
To enhance our viral loop and track B2B adoption across Cloud and Standalone modes, we need an automated way to manage Team Invites. Currently, there is no aggregated metric API to expose this to internal dashboards.

## Research Report
The growth strategy audit indicates that team-based expansion is a significant growth lever. A team referral API enables continuous monitoring of inviter-invitee relationships and conversion flows.

## Design Doc
1. Add a `TeamInvite` struct.
2. Add a `handleTeamInvites` HTTP endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Register the endpoint `/api/growth/team-invites` in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation
1. Implemented the `TeamInvite` model and handlers.
2. Updated the central `Server` struct.
3. Achieved test coverage for GET and POST methods.
