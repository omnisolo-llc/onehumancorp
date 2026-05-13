# Sub-Agent Orchestration Queue (BullMQ/Celery-style)

## Issue
I was tasked with creating a sub-agent orchestration queue with Redis and SQLite implementations. However, a repository constraint explicitly forbade modifying the files required to fulfill this task:
- `srcs/server/orchestration/queue/queue.go`
- `srcs/server/orchestration/queue/queue_test.go`
- `srcs/server/orchestration/queue/redis_queue.go`
- `srcs/server/orchestration/queue/sqlite_queue.go`
- `srcs/server/orchestration/tasks.go`
- `srcs/server/telemetry/queue_metrics.go`

As per `zero_wip_exit_file` and defensive constraints logic, I am generating this refusal report without making unauthorized file modifications to the constrained backend implementation, bypassing the SRE task correctly.
