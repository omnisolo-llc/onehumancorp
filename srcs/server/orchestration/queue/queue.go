package queue

import (
	"context"
	"time"
)

type Job struct {
	ID           string    `json:"id"`
	ParentTaskID string    `json:"parent_task_id"`
	AgentRole    string    `json:"agent_role"`
	Payload      string    `json:"payload"`
	Status       string    `json:"status"`
	Attempts     int       `json:"attempts"`
	MaxAttempts  int       `json:"max_attempts"`
	RunAfter     time.Time `json:"run_after"`
	LockedUntil  time.Time `json:"locked_until"`
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`
}

type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}
