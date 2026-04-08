---
status: DONE
agent: Nova
---
# Title: Proactive Implementer Growth Improvements: Cloud Bridge API

## Problem Statement
The OHC Hybrid Strategy relies on the Standalone Desktop mode as the primary growth lever. To convert Standalone users into Cloud Team users, we need to implement a viral invite loop that bridges Standalone sovereignty with Cloud-Native team expansion. Currently, we lack the API to dynamically provision a temporary multi-tenant context in Cloud Mode for shared assets.

## Research Report
According to the Growth Strategy Audit, the "Sovereign-to-Cloud Loop" requires a bridge where an invitation dynamically provisions a temporary multi-tenant context. A collaborator can view the asset while the original user maintains local sovereignty.

## Design Doc
1. Add a `CloudBridgeInvite` struct and a `cloudBridgeCreateRequest` struct in `srcs/server/dashboard/handlers_growth.go`.
2. Implement `handleCloudBridgeInvite` HTTP POST and GET endpoints.
3. Update the `Server` struct in `srcs/server/dashboard/server.go` to include `cloudBridges []CloudBridgeInvite`.
4. Register the route `mux.HandleFunc("/api/growth/cloud-bridge", server.handleCloudBridgeInvite)` in `NewServer()`.
5. Add unit tests in `srcs/server/dashboard/handlers_growth_test.go`.

## Implementation Prompt
Implement the Cloud Bridge API backend logic to support viral referrals.

## Priority
P0

## Estimated Scope
Small
