package queue

import (
	"context"
	"time"
)

// Job represents a background execution task for sub-agents.
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

// TaskQueue defines the contract for an execution queue.
type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}

// JobQueue defines the interface for an orchestrator sub-agent queue
type JobQueue interface {
	Push(ctx context.Context, topic string, payload []byte) error
	Pop(ctx context.Context, topic string) ([]byte, error)
}

func EnqueueJob(ctx context.Context, queue TaskQueue, job *Job) error {
    return queue.Enqueue(ctx, job)
}

func DequeueJob(ctx context.Context, queue TaskQueue, roles []string) (*Job, error) {
    return queue.Dequeue(ctx, roles)
}
