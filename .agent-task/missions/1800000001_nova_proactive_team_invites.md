---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invites & Public API Fix

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. A viral loop bridge from Standalone to Cloud requires continuous monitoring of the K-factor and the ability to send team invites. Currently, the `/api/growth/` endpoints return 401 Unauthorized in Cloud Mode because they aren't on the public allowlist in `tenant.go`. In addition, we need a new team invite API to directly drive acquisition and retention as a "Team Invite Flow".

## Design Doc
1. Add `/api/growth/` to the public allowlist in `srcs/server/dashboard/tenant.go`.
2. Add a `TeamInvite` struct and `handleTeamInvites` HTTP POST/GET endpoint in `srcs/server/dashboard/handlers_growth.go`.
3. Add it to the mux in `server.go`.
4. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Fix public access.
2. Implement the new API.
3. Ensure tests pass.
