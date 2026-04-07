---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Cloud Team Invites

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. We need to implement a mechanism to seamlessly invite users to a cloud tenant (bridging Standalone to Cloud), thereby reducing friction in team invites. This directly addresses the 18% conversion rate from Standalone User to Cloud Team User.

## Research Report
The `docs/growth_strategy_audit.md` states:
"Standalone User -> Cloud Team User: Friction in team invites. Intervention: Referral Engineering: Seamless Cloud bridging."

We already have a `Referral` concept, but we need a concrete `TeamInvite` resource that allows a Standalone user to invite a collaborator into a newly provisioned Cloud Tenant to view a shared context.

## Design Doc
1. Define a `TeamInvite` struct in `srcs/server/dashboard/handlers_growth.go`.
2. Add a `handleTeamInvites` HTTP POST/GET endpoint.
3. Hook it up in `srcs/server/dashboard/server.go`.
4. Add tests in `srcs/server/dashboard/handlers_growth_test.go`.

## Implementation Prompt
1. Add `TeamInvite`, `TeamInviteRequest`.
2. Ensure we expose `handleTeamInvites` via `/api/growth/team-invites`.
3. Add robust unit testing.
4. Use standard OHC styles and data structures.
