---
status: PENDING
Title: "KAIROS Phase 2: Realtime Teammate Mesh APIs"
Priority: P0
Estimated Scope: Medium
---

# Problem Statement
The One Human Corp (OHC) Swarm requires the Realtime Teammate Mesh APIs so other feature agents can implement them in production. A central "KAIROS" orchestrator layer is needed to decompose complex feature requests into a shared task list for the agent team. The Teammate Mesh ensures agents coordinate without delays.

# Research Report
Based on `CLAUDE_OHC.md` and `docs/features/kairos_orchestration.md`:
- The system operates in a Hybrid Architecture (`OHC-HA`).
- In Cloud-Native Mode, Redis Pub/Sub drives the Centrifuge WebSocket hubs (`mesh:tasks`, `mesh:coordination`).
- In Standalone Mode, In-Memory channel broadcast ensures low-latency IPC.
- Agents use `POST /api/mesh/broadcast` to announce task claims and updates.
- All updates sent to the Centrifuge channel must enforce the OHC-SIP JSON structure, guaranteeing that `agent_id`, `action`, and `status` reside at the root level.

# Design Doc
**Teammate Mesh API Architecture:**
1. **Endpoint**: `POST /api/mesh/broadcast`
2. **Payload**:
```json
{
  "agent_id": "string",
  "action": "CLAIM | COMPLETE | FAILED",
  "status": "string",
  "payload": { ... }
}
```
3. **Validation**: Enforce the OHC-SIP JSON structure. The request must contain `agent_id`, `action`, and `status`.
4. **Transport**: Use the Centrifuge hub (`tm.hub.PublishTaskBroadcast` or similar) to broadcast the message to the appropriate channel.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the Realtime Teammate Mesh APIs.
1. Implement the API handler for `POST /api/mesh/broadcast` in `srcs/server/orchestration/` (e.g., in a new `mesh.go` or adding to an existing file like `service.go`).
2. Add the route to the router in `srcs/server/orchestration/` or `srcs/server/` where API routes are defined.
3. Validate that the incoming JSON payload contains `agent_id`, `action`, and `status` at the root level.
4. If validation passes, broadcast the message using the Centrifuge hub (e.g., `hub.Publish` or a specific method for mesh broadcasting).
5. Ensure the API degrades gracefully in Standalone Mode (using the in-memory fallback of the hub).
6. Write unit tests for the new API endpoint, verifying both successful broadcasts and validation failures.
7. Use `bazelisk test //srcs/server/...` to verify your code.

# Visual Excellence Guidelines
Any frontend dashboards visualizing the Teammate Mesh must apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
