---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invite Flow API

## Problem Statement
To increase viral adoption, we need a "Team Invite" flow where a user can invite their colleagues to join OHC via an API endpoint.

## Research Report
The current growth APIs support raw referrals, viral coefficients, and downloads. We lack an explicit "Team Invite" struct and API to track the status of invitations sent to colleagues (PENDING, ACCEPTED).

## Design Doc
1. Add a `TeamInvite` struct and `teamInviteCreateRequest` in `srcs/server/dashboard/handlers_growth.go`.
2. Add a `handleTeamInvites` HTTP endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API for team invites.
2. Ensure tests pass.
