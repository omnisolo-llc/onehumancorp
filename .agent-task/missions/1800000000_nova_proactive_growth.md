---
status: DONE
agent: Nova
---
Title: Proactive Implementer Growth Improvements: Sovereign-to-Cloud Bridge API
Priority: P0
Estimated Scope: Medium
---

# Problem Statement
Based on the `docs/growth_strategy_audit.md`, the primary growth lever is the Standalone Mode and its conversion to Cloud Mode via the "Sovereign-to-Cloud Loop". Currently, there is no API endpoint to handle the provisioning of a "temporary multi-tenant context" when a referral link is generated.

# Research Report
The audit states: "The invitation dynamically provisions a temporary multi-tenant context in Cloud Mode, allowing the collaborator to view the asset while the original user maintains ultimate local sovereignty over the source data."
We need to implement this dynamic provisioning logic in the backend.

# Design Doc
1.  **Endpoint:** `POST /api/growth/bridge-context`
2.  **Request Body:**
    ```json
    {
      "inviterId": "string",
      "referralCode": "string",
      "assetId": "string"
    }
    ```
3.  **Response:**
    ```json
    {
      "temporaryTenantId": "string",
      "expiresAt": "timestamp",
      "status": "PROVISIONED"
    }
    ```
4.  **Logic:** The endpoint will generate a temporary tenant ID, set an expiration time (e.g., 24 hours), and store this state.

# Implementation Prompt
Dear Implementer Agent,
Please implement the Sovereign-to-Cloud Bridge Context API.
1. Add `BridgeContext` and `bridgeContextRequest` structs in `srcs/server/dashboard/handlers_growth.go`.
2. Add the `handleBridgeContext` endpoint to process the request.
3. Register the endpoint in `srcs/server/dashboard/server.go`.
4. Add unit tests in `srcs/server/dashboard/handlers_growth_test.go`.
