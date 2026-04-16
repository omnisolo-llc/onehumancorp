package queue

import (
	"context"
	"time"
)

// Task represents a unit of work to be executed by a sub-agent.
type Task struct {
	ID          string
	Command     string
	Args        []string
	Attempts    int
	MaxAttempts int
	RunAfter    time.Time
	LockedUntil time.Time
	Status      string
}

// TaskQueue defines the interface for task enqueuing and acquisition.
type TaskQueue interface {
	Enqueue(ctx context.Context, task *Task) error
	Acquire(ctx context.Context) (*Task, error)
	Complete(ctx context.Context, taskID string) error
	Fail(ctx context.Context, taskID string, retryAfter time.Duration) error
}
