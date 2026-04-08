---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Team Invite Quota / Cloud Bridge API

## Problem Statement
The growth strategy audit emphasizes that "The Standalone Mode is the Trojan Horse for Cloud-Native adoption." To bridge Standalone to Cloud via the Sovereign-to-Cloud referral loop, we need a backend endpoint that generates the actual Viral Invite Link for a user.

## Research Report
Currently, `handlers_growth.go` tracks referrals, viral coefficients, and downloads, but lacks an endpoint to generate a trackable team invite link.
Adding a `/api/growth/invite-link` endpoint will close this loop, allowing Standalone users to dynamically provision a secure multi-tenant context.

## Design Doc
1. Add an `InviteLinkResponse` struct.
2. Add an `InviteLinkRequest` struct.
3. Add a `handleInviteLink` HTTP POST endpoint in `srcs/server/dashboard/handlers_growth.go`.
4. Add it to the mux in `srcs/server/dashboard/server.go`.
5. Add unit tests in `handlers_growth_test.go`.

## Implementation Prompt
1. Implement the API endpoint.
2. Ensure tests pass.
