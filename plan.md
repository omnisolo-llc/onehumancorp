1. **Update `srcs/proto/hub.proto`**: (Looks like it is already updated, `MeshEvent`, `StreamMeshEvents`, `AgentCapabilities`, `AdvertiseCapabilities`, `DiscoverAgents` are all there). I will run `bazelisk test //srcs/proto/...` to verify.

2. **Update `TaskManager` (in `srcs/server/orchestration/tasks.go`)**:
   - The mission says: "Update `TaskManager` to utilize the new `MeshTransport` for broadcasting events via the `CentrifugeNode` hub."
   - Looking at `CentrifugeNode`, it has a method `PublishTaskBroadcast(taskID string, payload map[string]interface{})` that uses `MeshTransport`:
     ```go
     if cn.meshTransport != nil {
       data, err := json.Marshal(payload)
       if err == nil {
         _ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "tasks", data)
       }
     }
     ```
   - Oh, looking at `TaskManager`, it has `hub *CentrifugeNode`. Wait, maybe `TaskManager` is already utilizing it because it calls `tm.hub.PublishTaskBroadcast(taskID, payload)`?
     Yes! `tm.hub.PublishTaskBroadcast(taskID, payload)` exists.
   - Wait, `TaskManager` creates a broadcast func inside `NewTaskManager`. And then uses `hub.PublishTaskBroadcast`.
   - The issue states: "Instrument all API endpoints with OpenTelemetry metrics (`telemetry.Record...`) for mesh latency and message throughput. Note that in Cloud-Native mode, `telemetry.BufferMetricFunc` is nil, so sync endpoints must route directly to OpenTelemetry."
   - Oh, I need to check if `StreamMeshEvents` and `AdvertiseCapabilities` etc. have metrics.

Let's read `srcs/server/orchestration/service_mesh.go`.
