package queue

import (
	"context"
)

// Job represents a background task
type Job struct {
	ID      string
	TaskID  string
	Role    string
	Payload []byte
}

// Queue abstracts the background job queuing mechanism
type Queue interface {
	// EnqueueSubAgent adds a new sub-agent job to the queue
	EnqueueSubAgent(ctx context.Context, taskID string, role string, payload []byte) error

	// ProcessSubAgentJob allows workers to process jobs.
	// Typically, the implementation will block or loop until jobs are available.
	ProcessSubAgentJob(ctx context.Context, job *Job) error
}
