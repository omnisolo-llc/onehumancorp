---
status: "DONE"
agent: "jules"
Title: "KAIROS Phase 5: Universal Mesh Bridge & Cross-Swarm Coordination"
Priority: "P1"
Estimated Scope: "Large"
---

# Problem Statement
As One Human Corp (OHC) scales, the need for agents to coordinate across trust boundaries (e.g., between a user's Local Standalone swarm and their Company's Cloud swarm, or between two partner organizations) becomes critical. Currently, KAIROS orchestration is confined to a single organization/tenant. We need a "Universal Mesh Bridge" to securely route events, share tasks, and synchronize memory across independent KAIROS instances.

# Research Report
- **Inter-Swarm Communication:** Requires a secure, federated approach. Leveraging SPIFFE/SPIRE for cross-domain identity is the OHC standard.
- **Mesh Bridging:** The bridge must act as a selective proxy for Teammate Mesh (Centrifuge) events. Not all internal events should be bridged (Privacy vs. Coordination).
- **Hybrid Sync:** Standalone modes already have a "Sync Daemon" logic (see `srcs/server/orchestration/sync_daemon.go`). Phase 5 generalizes this into a persistent, multi-directional bridge.
- **Competitive Analysis:** Similar to NATS Leaf Nodes or Matrix Federation, but optimized for Agentic State Machines and DAG Task Lists.

# Design Doc
**Architecture:**
- **Bridge Manager:** A new service component in `srcs/server/orchestration/bridge.go` that manages outbound and inbound connections to remote swarms.
- **Selective Forwarding:** A configuration layer to define which topics (e.g., `mesh:tasks:shared`) are broadcasted across the bridge.
- **Cross-Org Task Delegation:** Extending the `SharedTask` model to support `origin_organization_id`.

**Database Schema:**
```sql
CREATE TABLE IF NOT EXISTS mesh_bridges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL, -- The local org owning this bridge
    remote_swarm_url VARCHAR NOT NULL,
    remote_organization_id VARCHAR NOT NULL,
    bridge_type VARCHAR NOT NULL DEFAULT 'P2P', -- P2P, RELAY, HIERARCHICAL
    status VARCHAR NOT NULL DEFAULT 'INACTIVE',
    metadata JSONB DEFAULT '{}', -- Stores allowed topics, rate limits, etc.
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_mesh_bridges_org ON mesh_bridges(organization_id);
```

**API Contracts:**
- `POST /api/v1/mesh/bridge/connect`: Initiates a handshake with a remote swarm.
- `GET /api/v1/mesh/bridge/status`: Returns health and throughput metrics for active bridges.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the "Universal Mesh Bridge" for KAIROS Phase 5.

1. **Schema:** Create a new SQL migration `032_mesh_bridges.sql` with the schema provided above. Add it to `srcs/server/db/BUILD.bazel`.
2. **Logic:** Create `srcs/server/orchestration/bridge_manager.go`.
   - Implement a `BridgeManager` that can establish a WebSocket connection to a remote OHC API.
   - Use the `CentrifugeNode` to listen for local events on "bridgeable" topics and forward them to the remote connection.
   - Handle inbound events from the remote connection by re-broadcasting them to the local mesh.
3. **Security:** Ensure all bridge connections are authenticated using SPIFFE SVIDs. Use the existing mTLS interceptor logic for verification.
4. **Resiliency:** Implement exponential backoff for bridge reconnections.
5. **Observability:** Record metrics: `ohc_mesh_bridge_messages_sent_total`, `ohc_mesh_bridge_messages_received_total`, and `ohc_mesh_bridge_status_gauge`.
6. **Tests:** Write unit tests in `srcs/server/orchestration/bridge_manager_test.go` simulating two independent `TaskManager` instances connected via a bridge.
7. **Verification:** Ensure `bazelisk test //srcs/server/orchestration/...` passes with >90% coverage on the new bridge logic.

# Visual Excellence Guidelines
The "Bridge Status" dashboard must be integrated into the CEO UI using:
`<style>
.bridge-card {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
}
</style>`
