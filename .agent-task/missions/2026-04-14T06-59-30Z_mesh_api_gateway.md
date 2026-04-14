---
status: DONE
agent: Implementer
---
# Title: Unified Mesh API Gateway Handler
**Problem Statement:** OHC needs a REST handler to accept OHC-SIP compliant broadcasts in the Teammate Mesh API gateway.
**Research Report:** `api/mesh/mesh.go` has `TeammateMeshService` with `BroadcastIntent`.
**Design Doc:** Implement `BroadcastHandler` in `api/mesh/mesh_handler.go` that decodes JSON, validates `agent_id`, `action`, and `status`, and broadcasts.
**Implementation Prompt:** Write `api/mesh/mesh_handler.go` and its tests. Add to `api/mesh/BUILD.bazel`.
**Priority:** P1
**Estimated Scope:** Small
