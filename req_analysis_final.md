I'm ready to write the plan and submit it for review.

1. `hub.go`: Add `PublishTeammateMeshEvent` to `Hub` struct.
2. `service.go`: Update `HubServiceServer.PublishTeammateMeshEvent` to use the new method on `s.hub`.
3. `auth_interceptor.go`: Add `pb.PublishTeammateMeshEventRequest` validation logic.
4. `task_orchestrator.go`: Change `queue.NewPostgresTaskQueue(provider)` to `queue.NewRedisTaskQueue(redisClient, "")`.
5. Pre-commit test step: `bazelisk test //srcs/server/orchestration/...`

Let's do this.
