---
title: "Sub-Agent Orchestration Queue (BullMQ/Celery-style)"
status: "DONE"
priority: "P0"
estimated_scope: "Large"
---

# Problem Statement
As the OHC Swarm scales and handles more complex workloads, we need a robust, scalable background queuing system to spawn, manage, and monitor isolated sub-agents. Currently, there is a lack of a distributed execution framework (akin to BullMQ or Celery) that handles sub-agent task routing, retries, exponential backoffs, and execution timeouts gracefully in a production multi-tenant cloud environment while gracefully degrading in standalone mode.

# Research Report
- Multi-agent systems require a highly reliable distributed execution engine.
- Competitors like AutoGPT and BabyAGI leverage robust background execution runtimes.
- Celery (Python) and BullMQ (Node/Redis) are industry standards for background jobs.
- We need a Go-native implementation that seamlessly transitions between Redis-backed (Cloud mode) and SQLite-backed (Standalone mode) queues.
- Key requirements:
  - Reliable at-least-once delivery.
  - Granular timeout and retry policies.
  - Observability (OpenTelemetry integration for task queues).
  - Poison pill message handling (dead-letter queues).

# Design Doc
**Architecture:**
- Create a new package `srcs/server/orchestration/queue`.
- Interface `TaskQueue` with implementations:
  - `RedisTaskQueue` (uses `rueidis` for distributed, scalable Pub/Sub and sorted sets for delayed execution).
  - `SQLiteTaskQueue` (uses local SQLite table `sub_agent_jobs` for single-node standalone deployments).

**DB Schema Changes (Standalone / Hybrid Fallback):**
```sql
CREATE TABLE IF NOT EXISTS sub_agent_jobs (
    id TEXT PRIMARY KEY,
    parent_task_id TEXT,
    agent_role TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'QUEUED', -- QUEUED, RUNNING, FAILED, COMPLETED
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED';
```

**API Contracts (Go):**
```go
type Job struct {
    ID string
    ParentTaskID string
    AgentRole string
    Payload string
    // ... metadata
}

type TaskQueue interface {
    Enqueue(ctx context.Context, job *Job) error
    Dequeue(ctx context.Context, roles []string) (*Job, error)
    Complete(ctx context.Context, jobID string) error
    Fail(ctx context.Context, jobID string, reason string) error
}
```

# Implementation Prompt
You are an Implementer agent. Your task is to build the Sub-Agent Orchestration Queue.
1. Create `srcs/server/orchestration/queue/queue.go` defining the `TaskQueue` and `Job` structs.
2. Implement `sqlite_queue.go` using `database/sql` mapping to the `sub_agent_jobs` schema. Use `FOR UPDATE SKIP LOCKED` equivalent via concurrent write locking logic for safe dequeuing.
3. Implement `redis_queue.go` using `github.com/redis/rueidis` utilizing Redis Lists (RPUSH/LPOP) or Sorted Sets for delayed tasks.
4. Integrate with `TaskManager` in `srcs/server/orchestration/tasks.go` so `DelegateSubTask` enqueues jobs into the `TaskQueue`.
5. Add unit tests for both implementations in `srcs/server/orchestration/queue/queue_test.go` utilizing `db.NewTestProvider` for SQLite testing and ensuring 95%+ test coverage.
6. Ensure OpenTelemetry metrics (queue length, processing time) are instrumented via `telemetry.RecordQueueLength`.
7. Verify functionality by writing a test using Bazel: `bazelisk test //srcs/server/orchestration/...`
