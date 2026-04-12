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

// genericBackgroundJobFramework outlines a basic generic background job framework as requested by Phase 4.
type GenericBackgroundJob struct {
	ID      string
	Payload string
	Status  string
}

type GenericJobWorker struct {
	Queue TaskQueue
}

func (w *GenericJobWorker) Start(ctx context.Context, roles []string) {
	ticker := time.NewTicker(time.Second * 5)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			job, err := w.Queue.Dequeue(ctx, roles)
			if err == nil && job != nil {
				// Process job
				w.Queue.Complete(ctx, job.ID)
			}
		}
	}
}
