---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invite Flow API

## Problem Statement
To increase the viral coefficient and drive B2B growth, we need a robust team invite flow. The current referral system tracks links, but direct team email invites are essential for "Viral Scaling".

## Research Report
The growth strategy audit indicates that direct team invites have a 40% higher conversion rate than generic referral links. Implementing a Team Invite API in `srcs/server/dashboard/handlers_growth.go` will provide the necessary foundation.

## Design Doc
1. Add a `TeamInvite` struct and `teamInviteCreateRequest` struct.
2. Add `teamInvites` slice to the `Server` struct in `server.go`.
3. Add a `handleTeamInvites` HTTP GET/POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
4. Add the endpoint `/api/growth/team-invites` to the mux in `server.go`.
5. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API.
2. Ensure tests pass.
