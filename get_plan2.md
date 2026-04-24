So `redis_queue.go` has `RedisTaskQueue` fully implemented. `sqlite_queue.go` has `SQLiteTaskQueue`. Both use the same queue model (`TaskQueue`).
The user prompt says:
"2. Implement sub-agent background queuing logic fetching from sub_agent_jobs using Redis or SQLite fallbacks."

Wait! There's an `orchestration.TaskManager` which uses `queue.TaskQueue` maybe? No, `task_orchestrator.go` uses `queue.TaskQueue`.
Let's read `task_orchestrator.go`.
