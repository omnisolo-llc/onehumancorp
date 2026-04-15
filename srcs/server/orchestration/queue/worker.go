package queue

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// JobHandler is a function that processes a job.
type JobHandler func(ctx context.Context, job *Job) error

// Worker represents a background worker that processes jobs from a TaskQueue.
type Worker struct {
	queue   TaskQueue
	roles   []string
	handler JobHandler
}

// NewWorker creates a new Worker instance.
func NewWorker(q TaskQueue, roles []string, handler JobHandler) *Worker {
	return &Worker{
		queue:   q,
		roles:   roles,
		handler: handler,
	}
}

// Start begins the worker loop, polling the queue for jobs.
// It blocks until the context is canceled.
func (w *Worker) Start(ctx context.Context) {
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			job, err := w.queue.Dequeue(ctx, w.roles)
			if err != nil {
				slog.Error("Worker failed to dequeue job", "error", err)
				continue
			}

			if job == nil {
				// No job available
				continue
			}

			delay := time.Since(job.CreatedAt).Seconds()
			telemetry.RecordSubAgentQueueDelay(ctx, delay)

			slog.Info("Worker processing job", "job_id", job.ID)
			err = w.handler(ctx, job)
			if err != nil {
				slog.Error("Worker failed to process job", "job_id", job.ID, "error", err)
				if fErr := w.queue.Fail(ctx, job.ID, err.Error()); fErr != nil {
					slog.Error("Worker failed to mark job as failed", "job_id", job.ID, "error", fErr)
				}
			} else {
				slog.Info("Worker successfully processed job", "job_id", job.ID)
				if cErr := w.queue.Complete(ctx, job.ID); cErr != nil {
					slog.Error("Worker failed to mark job as complete", "job_id", job.ID, "error", cErr)
				}
			}
		}
	}
}
