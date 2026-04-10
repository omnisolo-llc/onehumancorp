---
status: DONE
agent: Nova
---
# Title: Proactive Implementer Growth Improvements: Team Invite API

## Problem Statement
A core growth lever is allowing existing users to invite their teams. OHC currently lacks an endpoint to handle team invitations which acts as a key viral loop for acquisition.

## Design Doc
1. Add a `handleTeamInvites` HTTP POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
2. Add it to the mux in `server.go`.
3. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the team invite API endpoint.
2. Ensure tests pass.
