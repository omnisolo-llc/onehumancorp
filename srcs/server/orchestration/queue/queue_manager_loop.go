package queue

import (
	"context"
	"log/slog"
	"time"
)

type SubAgentJobHandler func(ctx context.Context, job *SubAgentJob) error

// StartPolling starts a polling loop that fetches jobs from the queue and dispatches them to the provided handler.
func (q *QueueManager) StartPolling(ctx context.Context, workerID string, interval time.Duration, handler SubAgentJobHandler) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Continuously poll until queue is empty
			for {
				job, err := q.Poll(ctx, workerID)
				if err != nil {
					slog.Error("Failed to poll queue manager", "error", err, "worker_id", workerID)
					break
				}

				if job == nil {
					// No more jobs in queue, wait for next tick
					break
				}

				slog.Info("Dispatched sub-agent job", "job_id", job.ID, "worker_id", workerID)

				// Dispatch the job
				err = handler(ctx, job)
				if err != nil {
					slog.Error("Job handler failed", "job_id", job.ID, "error", err)
					q.MarkFailed(ctx, job.ID, err.Error())
				} else {
					q.MarkCompleted(ctx, job.ID)
				}
			}
		}
	}
}

// MarkCompleted updates the job status to COMPLETED
func (q *QueueManager) MarkCompleted(ctx context.Context, jobID string) error {
	query := `UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = $1 WHERE id = $2`
	_, err := q.provider.Exec(ctx, query, time.Now(), jobID)
	return err
}

// MarkFailed updates the job status to FAILED
func (q *QueueManager) MarkFailed(ctx context.Context, jobID string, reason string) error {
	query := `UPDATE sub_agent_queue SET status = 'FAILED', updated_at = $1 WHERE id = $2` // In a real system, we'd also store the reason
	_, err := q.provider.Exec(ctx, query, time.Now(), jobID)
	return err
}
