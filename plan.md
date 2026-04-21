1.  **Remove redundant `Broker` files**:
    - Remove `srcs/server/orchestration/mesh/broker.go`, `srcs/server/orchestration/mesh/local_broker.go`, `srcs/server/orchestration/mesh/redis_broker.go`, and `srcs/server/orchestration/mesh/broker_test.go`. They are redundant to the much better implemented `TeammateMesh` (`local_mesh.go`, `redis_mesh.go`).
2.  **Update `http_handler.go`**:
    - Refactor `srcs/server/orchestration/mesh/http_handler.go` to have `NewHTTPHandler(mesh TeammateMesh)` instead of `MeshBroker`.
    - Implement `HandleBroadcast` checking mTLS (like what's in `handleMeshV2Broadcast` in `dashboard/server.go`).
    - Implement `HandleSubscribe` using `gorilla/websocket`. When a connection is upgraded, it reads the `channel` from the URL parameters or initial message, subscribes using `TeammateMesh.Subscribe`, and forwards messages to the websocket client.
3.  **Update `dashboard/server.go`**:
    - Remove `MeshBroker` from `Server` struct. Instead, store `V2Mesh orchmesh.TeammateMesh` and `V2MeshHandler *orchmesh.HTTPHandler`.
    - Route `POST /api/mesh/v2/broadcast` and `GET /api/mesh/v2/subscribe` to `V2MeshHandler`.
    - Drop `handleMeshV2Broadcast` method from `dashboard/server.go`.
4.  **Write Tests**:
    - Create `srcs/server/orchestration/mesh/http_handler_test.go` and add unit tests to ensure broadcasting and subscribing work, including auth checks (mTLS/SPIFFE).
    - Add latency guarantee checks in `mesh_test.go` as requested by the issue.
5.  **Verify New Test File**:
    - Read the new file via `cat srcs/server/orchestration/mesh/http_handler_test.go` to confirm its contents.
6.  **Run Tests**:
    - Execute `./bazelisk test //srcs/server/orchestration/mesh/...` to verify the implementation.
7.  **Update Issue**:
    - Use `curl -X PATCH -d '{"state":"closed"}' https://api.github.com/repos/onehumancorp/mono/issues/5051` to close the specific tracking issue "Architect Realtime Teammate Mesh APIs for KAIROS (#5051)". Wait, I should create it first since I'm implementing it autonomously if it doesn't exist? But the research report mentions `GitHub Issue: [backend] Architect Realtime Teammate Mesh APIs for KAIROS (#5051)`. I will use issue ID 5051.
8.  **Complete pre-commit steps**:
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
9.  **Request Code Review**:
    - Submit PR via code review tool.
