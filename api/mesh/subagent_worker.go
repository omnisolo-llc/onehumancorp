package mesh

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

// JobExecutor defines the interface for executing dequeued sub-agent jobs.
type JobExecutor interface {
	Execute(ctx context.Context, job *queue.Job) error
}

// SubAgentWorker polls a TaskQueue and dispatches jobs to a JobExecutor.
type SubAgentWorker struct {
	queue    queue.TaskQueue
	executor JobExecutor
	roles    []string
	interval time.Duration
	stop     chan struct{}
}

// NewSubAgentWorker creates a new SubAgentWorker.
func NewSubAgentWorker(q queue.TaskQueue, exec JobExecutor, roles []string, interval time.Duration) *SubAgentWorker {
	if interval == 0 {
		interval = 5 * time.Second
	}
	return &SubAgentWorker{
		queue:    q,
		executor: exec,
		roles:    roles,
		interval: interval,
		stop:     make(chan struct{}),
	}
}

// Start runs the worker polling loop until the context is canceled or Stop is called.
func (w *SubAgentWorker) Start(ctx context.Context) {
	slog.Info("starting sub-agent worker", "roles", w.roles, "interval", w.interval)
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-w.stop:
			return
		case <-ticker.C:
			w.poll(ctx)
		}
	}
}

// Stop signals the worker loop to exit.
func (w *SubAgentWorker) Stop() {
	close(w.stop)
}

func (w *SubAgentWorker) poll(ctx context.Context) {
	job, err := w.queue.Dequeue(ctx, w.roles)
	if err != nil {
		slog.Error("failed to dequeue sub-agent job", "error", err)
		return
	}
	if job == nil {
		return
	}

	slog.Info("processing sub-agent job", "job_id", job.ID, "role", job.AgentRole)

	// Execute job asynchronously to not block the polling loop.
	// In a production scenario, we might want a worker pool here.
	go func(j *queue.Job) {
		err := w.executor.Execute(ctx, j)
		if err != nil {
			slog.Error("job execution failed", "job_id", j.ID, "error", err)
			// Attempt to mark the job as failed in the queue
			if failErr := w.queue.Fail(ctx, j.ID, err.Error()); failErr != nil {
				slog.Error("failed to mark job as failed in queue", "job_id", j.ID, "error", failErr)
			}
		} else {
			slog.Info("job execution completed", "job_id", j.ID)
			// Mark job as completed
			if compErr := w.queue.Complete(ctx, j.ID); compErr != nil {
				slog.Error("failed to mark job as complete in queue", "job_id", j.ID, "error", compErr)
			}
		}
	}(job)
}
