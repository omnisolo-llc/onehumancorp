package queue

import (
    "context"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type Job struct {
    ID           string
    ParentTaskID string
    AgentRole    string
    Payload      string
    Status       string
    Attempts     int
    MaxAttempts  int
    RunAfter     time.Time
    LockedUntil  *time.Time
    CreatedAt    time.Time
    UpdatedAt    time.Time
}

type TaskQueue interface {
    Enqueue(ctx context.Context, job *Job) error
    Dequeue(ctx context.Context, roles []string) (*Job, error)
    Complete(ctx context.Context, jobID string) error
    Fail(ctx context.Context, jobID string, reason string) error
}

type JobQueue interface {
    Push(ctx context.Context, topic string, payload []byte) error
    Pop(ctx context.Context, topic string) ([]byte, error)
}

func EnqueueJob(ctx context.Context, pool db.Provider, job Job) error {
    if pool.IsSQLite() {
        queue := NewSQLiteTaskQueue(pool)
        return queue.Enqueue(ctx, &job)
    }
    return nil
}

func DequeueJob(ctx context.Context, pool db.Provider) (*Job, error) {
    if pool.IsSQLite() {
        queue := NewSQLiteTaskQueue(pool)
        return queue.Dequeue(ctx, []string{})
    }
    return nil, nil
}
