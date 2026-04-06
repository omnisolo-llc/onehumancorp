package queue

import (
	"context"
)

type Job struct {
	ID           string
	ParentTaskID string
	AgentRole    string
	Payload      string
	Attempts     int
	MaxAttempts  int
}

type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}
