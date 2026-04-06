package queue

import (
	"context"
	"time"
)

// Job represents a sub-agent task enqueued for background processing.
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

// TaskQueue defines the distributed execution engine interface for sub-agents.
type TaskQueue interface {
	// Enqueue adds a new job to the queue.
	Enqueue(ctx context.Context, job *Job) error

	// Dequeue attempts to fetch and lock an available job that matches one of the specified roles.
	// If no roles are specified, it may fetch any available job.
	// Returns nil, nil if no jobs are available.
	Dequeue(ctx context.Context, roles []string) (*Job, error)

	// Complete marks a job as successfully completed.
	Complete(ctx context.Context, jobID string) error

	// Fail marks a job as failed, incrementing the attempt count.
	// If attempts exceed max attempts, it should transition to a permanently failed state.
	Fail(ctx context.Context, jobID string, reason string) error
}
