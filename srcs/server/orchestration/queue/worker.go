package queue

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// HandlerFunc is the signature for the agent's actual logic.
type HandlerFunc func(ctx context.Context, job *Job) error

// Worker represents a single queue worker.
type Worker struct {
	queue    TaskQueue
	roles    []string
	handler  HandlerFunc
	interval time.Duration
}

// NewWorker creates a new worker for the queue.
func NewWorker(q TaskQueue, roles []string, handler HandlerFunc, pollInterval time.Duration) *Worker {
	if pollInterval == 0 {
		pollInterval = 1 * time.Second
	}
	return &Worker{
		queue:    q,
		roles:    roles,
		handler:  handler,
		interval: pollInterval,
	}
}

// Start starts the worker loop.
func (w *Worker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			slog.Info("worker stopped", "reason", ctx.Err())
			return
		default:
		}

		job, err := w.queue.Dequeue(ctx, w.roles)
		if err != nil {
			slog.Error("failed to dequeue job", "error", err)
			time.Sleep(w.interval)
			continue
		}

		if job == nil {
			// No jobs available
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
			}
			continue
		}

		// Found a job, record metrics
		telemetry.RecordWorkerJobStart(ctx)

		slog.Info("processing job", "job_id", job.ID, "parent_task_id", job.ParentTaskID)

		err = w.handler(ctx, job)
		if err != nil {
			telemetry.RecordWorkerJobComplete(ctx, false)
			slog.Error("job handler failed", "job_id", job.ID, "error", err)
			failErr := w.queue.Fail(ctx, job.ID, err.Error())
			if failErr != nil {
				slog.Error("failed to mark job as failed", "job_id", job.ID, "error", failErr)
			}
		} else {
			telemetry.RecordWorkerJobComplete(ctx, true)
			completeErr := w.queue.Complete(ctx, job.ID)
			if completeErr != nil {
				slog.Error("failed to mark job as complete", "job_id", job.ID, "error", completeErr)
			} else {
				slog.Info("job completed successfully", "job_id", job.ID)
			}
		}
	}
}
