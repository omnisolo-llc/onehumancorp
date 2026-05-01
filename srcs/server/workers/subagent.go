package workers

import (
	"context"

	"ohc/server/queue"
)

// SubAgentWorker handles the processing of sub-agent tasks
type SubAgentWorker struct {
	q queue.Queue
}

// NewSubAgentWorker creates a worker with a given queue implementation
func NewSubAgentWorker(q queue.Queue) *SubAgentWorker {
	return &SubAgentWorker{
		q: q,
	}
}

// HandleJob executes the sub-agent logic
func (w *SubAgentWorker) HandleJob(ctx context.Context, job *queue.Job) error {


	// Simulate execution context initialization
	// e.g. initialize LLM context, tool access, reporting progress back to Shared Task List

	// Process via queue logic (e.g. mark done in SQLite)
	if err := w.q.ProcessSubAgentJob(ctx, job); err != nil {
		return fmt.Errorf("failed to process job: %w", err)
	}

	return nil
}
