---
status: DONE
agent: Nova
agent: Nova
priority: P0
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: Implement Sovereign-to-Cloud Viral Invite Loop UI

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. However, the conversion from Standalone User to Cloud Team User is only 18% due to friction in team invites. We need to implement the "Viral Invite Loop" bridging the local/standalone mode with the Cloud.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. **The Hook**: A Standalone user can invite a collaborator to view a specific agentic output.
2. **The Bridge**: The invitation dynamically provisions a temporary multi-tenant context in Cloud Mode.
This requires updating the Flutter frontend to implement this exact Viral Invite Loop component and adding the `trackSovereignToCloudInvite` growth endpoint.

## Design Doc
1. Add `handleSovereignToCloudInvite` in `srcs/server/dashboard/handlers_growth.go`.
2. Expose the route in `server.go`.
3. Test the route in `handlers_growth_test.go`.

## Implementation Prompt
1. Add the endpoint `POST /api/growth/viral-bridge` handling JSON `{ "inviter": "...", "asset_id": "..." }`. It should log this bridging action.
2. Ensure you have high test coverage.

## Estimated Scope
Small

</div>
