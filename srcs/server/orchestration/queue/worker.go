package queue

import (
	"context"
	"time"
)

// HandlerFunc is the logic to execute for a given job.
type HandlerFunc func(ctx context.Context, job *Job) error

// WorkerLoop continuously polls the queue and processes jobs.
func WorkerLoop(ctx context.Context, queue *QueueManager, roles []string, handler HandlerFunc) {
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		default:
		}

		job, err := queue.Poll(ctx, roles)
		if err != nil || job == nil {
			// Wait before polling again if queue is empty or errors occur
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
			}
			continue
		}

		err = handler(ctx, job)
		if err != nil {
			_ = queue.Fail(ctx, job.ID, err.Error())
		} else {
			_ = queue.Complete(ctx, job.ID)
		}
	}
}
