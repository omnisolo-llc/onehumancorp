# Plan

1. **Modify `Hub` inside `hub.go` to provide a way to publish teammate mesh events using `CentrifugeNode`**:
   Currently, the issue mentions: "Expand HubService in srcs/server/orchestration/hub.go to support realtime events via CentrifugeNode and RedisMeshTransport or MemoryMeshTransport."
   We should add `PublishTeammateMeshEvent` to `Hub` struct in `hub.go`:
   ```go
   func (h *Hub) PublishTeammateMeshEvent(agentID, action, status string, payload map[string]interface{}) {
       if h.centrifugeNode != nil {
           h.centrifugeNode.PublishTeammateMeshEvent(agentID, action, status, payload)
       }
   }
   ```

2. **Modify `HubServiceServer`'s `PublishTeammateMeshEvent` in `service.go`**:
   Currently it only uses `s.mesh.PublishTeammateMeshEvent`. We will modify it to also try `s.hub.PublishTeammateMeshEvent`:
   ```go
	event := req.GetEvent()
	var payloadMap map[string]interface{}
	if err := json.Unmarshal(event.GetPayload(), &payloadMap); err != nil {
		payloadMap = map[string]interface{}{}
	}
	if s.hub != nil {
		s.hub.PublishTeammateMeshEvent(event.GetAgentId(), event.GetAction(), event.GetStatus(), payloadMap)
	} else if s.mesh != nil {
		_ = s.mesh.PublishTeammateMeshEvent(ctx, req.GetChannel(), event.GetAgentId(), event.GetAction(), event.GetStatus(), event.GetPayload())
	}
   ```
   Wait, if `CentrifugeNode` internally calls `meshTransport`, `s.hub.PublishTeammateMeshEvent` will invoke `CentrifugeNode` which will invoke `meshTransport`.
   But wait, `HubServiceServer.PublishTeammateMeshEvent` uses `req.GetChannel()`, which is important for `SubscribeTeammateMesh`.

3. **Secure endpoints with SPIFFE interceptors**:
   Modify `srcs/server/orchestration/auth_interceptor.go`:
   Add `case *pb.PublishTeammateMeshEventRequest:` in `SPIFFEAuthInterceptor`:
   ```go
		case *pb.PublishTeammateMeshEventRequest:
			reqFromAgent := v.GetEvent().GetAgentId()
			if agentID != reqFromAgent {
				return nil, status.Errorf(codes.PermissionDenied, "SPIFFE ID %s cannot publish teammate event as agent %s", spiffeID, reqFromAgent)
			}
   ```
   In `SPIFFEStreamInterceptor` -> `RecvMsg`, we probably don't need to add anything since `EventStreamRequest` doesn't have an `agent_id` parameter to spoof, but we could add:
   ```go
	if req, ok := m.(*pb.EventStreamRequest); ok {
		// EventStreamRequest doesn't define agentId, any authenticated agent can stream
        _ = req
	}
   ```
   Actually no need, the auth interceptor already ensures the SPIFFE ID is valid.

4. **Implement sub-agent background queuing logic fetching from `sub_agent_jobs` using Redis or SQLite fallbacks.**
   In `srcs/server/orchestration/queue/sqlite_queue.go`, it already implements `SQLiteTaskQueue`.
   Wait, is `RedisTaskQueue` in `srcs/server/orchestration/queue/redis_queue.go` fully implemented?
   Let's check `Dequeue` and `Complete` in `redis_queue.go`!
   Let's check `Hub` or `SubAgentWorker` in `task_orchestrator.go` or `service.go` to see if `queue.TaskQueue` is provided!
