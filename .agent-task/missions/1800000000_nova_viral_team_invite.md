---
status: DONE
agent: Nova
---
# Title: Viral Loop Bridge: Team Invite Flow API

## Problem Statement
OHC currently lacks a native, friction-free way for users in the Standalone or Cloud environments to invite their team members, which is a key growth lever to increase our Viral Coefficient. As part of the OHC Hybrid Agentic OS, we need a robust API endpoint to handle team invites that will natively bridge into a viral loop.

## Research Report
The most effective way to trigger a viral loop is through native invitations within the product. We need a backend API that can receive a batch of emails to invite to an organization. For OHC-HA, this should be scoped under the Cloud-Native Postgres orchestration engine, although the endpoint should gracefully accept requests from Standalone Mode.

## Design Doc
1. Add an `InviteRequest` struct representing a batch of emails.
2. Implement an `inviteTeam` HTTP POST endpoint at `/api/growth/invite`.
3. Add it to the main router in `srcs/server/dashboard/handlers_growth.go`.
4. Ensure adequate unit test coverage in `handlers_growth_test.go`.

## Implementation Prompt
Implement the API for the Team Invite Flow to drive the Viral Loop. Write the required tests, ensure they pass with Bazel, and adhere to OHC-SIP.
