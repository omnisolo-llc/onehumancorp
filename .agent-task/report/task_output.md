# Teammate Mesh APIs for KAIROS

## Research Report
- **Goal**: Architect and implement Realtime Teammate Mesh APIs to enable low-latency coordination across the OHC swarm, reducing dependency on slow database polling.
- **Capabilities**:
  - Expose a `POST /api/mesh/v2/broadcast` endpoint for publishing events.
  - Establish subscription and mesh capabilities via the existing `meshTransport` paradigm for sub-millisecond, push-based event propagation using strict Protobuf payloads.
  - Accommodate Hybrid Fallbacks automatically (Redis in Cloud-Native via `OHC_MULTITENANT`, and In-Memory Go channels for Standalone).

## Design Doc
1. **Architecture Update**: Added `/api/mesh/v2/broadcast` handler avoiding duplicate or mock logic in the orchestration directory directly since `MeshAPI` already handles API routing logic. Added route directly via `srcs/server/orchestration/mesh_api.go`.
2. **Implementation details**: Added `HandleMeshV2Broadcast` under `srcs/server/orchestration/mesh_api.go`. Handled requests conform to the `pb.PublishTeammateMeshEventRequest` struct requirement over the wire per strict proto format, returning standardized HTTP `200 OK` or errors appropriately. Tests fully updated in `srcs/server/orchestration/mesh_api_test.go` and mock structs properly updated tracking invocations.
3. **Execution**: Full code implementation completed and confirmed with `bazelisk test //srcs/server/orchestration/...` achieving passing build limits with existing tests validating hybrid fallbacks in orchestration directory. Coverage manually validated.
