1. **Understand Problem:** We need to implement Realtime Teammate Mesh APIs to support task broadcasting for CentrifugeNode, bridging bare WebSockets and Redis Pub/Sub in production.
2. **Review Memory & Files:**
    - The mission (`.agent-task/missions/2026-04-06T08:42:36+00:00_kairos_teammate_mesh_apis.md`) outlines:
        - Expand `srcs/proto/hub.proto` (done).
        - Implement `MeshTransport` interface with Redis and Memory variants in `srcs/server/orchestration/hub.go` or `mesh.go` (done).
        - Update `TaskManager` to utilize the new `MeshTransport` for broadcasting events via the `CentrifugeNode` hub (done).
        - Instrument API endpoints with OpenTelemetry metrics (`telemetry.RecordMeshLatency` and `telemetry.RecordMeshBroadcast`), accounting for `BufferMetricFunc` being nil in Cloud-Native mode (done in `StreamMeshEvents`).
3. **Verify Existing Changes:**
    - I've modified `srcs/server/orchestration/service.go` to add `StreamMeshEvents` OpenTelemetry tracking.
    - I've modified `srcs/server/orchestration/tasks.go` and `task_orchestrator.go` to thread `mesh MeshTransport` through to `TaskManager`.
    - I've fixed `srcs/server/orchestration/service_test.go` to compile with the new `NewTaskManager` signature.
    - Tests `bazelisk test //srcs/server/orchestration/...` pass successfully.
4. **Finalize Code Check & Formatting:**
    - Ensure code formatting with `gofmt -w` (done).
    - Ensure `.agent-task/missions/` mission is set to `status: "DONE"` and `agent: "jules"` (done).
    - Provide a status heartbeat in `.agent-task/status/` (done).
5. **Pre-commit Steps:**
    - Run the pre commit instructions step to make sure verification steps are completed successfully.
6. **Submit:**
    - Submit PR with the new feature implementations.
