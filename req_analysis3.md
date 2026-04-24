Requirement 2: "Implement sub-agent background queuing logic fetching from sub_agent_jobs using Redis or SQLite fallbacks."

We saw `SubAgentTaskQueue` in `queue.go`:
```go
type SubAgentTaskQueue interface {
	Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error
	Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error)
}
```
Wait, there are already implementations for this!
`redis_subagent_queue.go` implements it using Redis.
`sqlite_subagent_queue.go` implements it using `sub_agent_tasks`! But wait, `sqlite_subagent_queue.go` uses `sub_agent_tasks` table!
Wait, look at `sqlite_subagent_queue.go`:
```go
	query := "INSERT INTO sub_agent_tasks (job_id, queue_name, payload, status, created_at) VALUES ($1, $2, $3, 'QUEUED', $4)"
...
		query := "SELECT job_id, payload FROM sub_agent_tasks WHERE status = 'QUEUED' AND queue_name = $1 ORDER BY created_at ASC LIMIT 1"
```
Wait! The database schema `028_sub_agent_jobs.sql` has `sub_agent_jobs` table.
And `sqlite_queue.go` has `SQLiteTaskQueue` that inserts into `sub_agent_jobs`.
Wait! The issue says: "Implement sub-agent background queuing logic fetching from sub_agent_jobs using Redis or SQLite fallbacks."

Let's look at `sub_agent.go` and `subagent_worker.go`.
Ah, `TaskQueue` interface uses `Job` struct.
`TaskQueue` has `Enqueue(ctx context.Context, job *Job) error` and `Dequeue(ctx context.Context, roles []string) (*Job, error)`.
`sqlite_queue.go` implements `TaskQueue` using `sub_agent_jobs`.
`postgres_queue.go` implements `TaskQueue` using `sub_agent_queue`. Wait! `postgres_queue.go` uses `sub_agent_queue`!
Let's see what `redis_queue.go` uses. It probably uses `TaskQueue` with redis? Let's check `redis_queue.go`.

Let's check `redis_queue.go`.
