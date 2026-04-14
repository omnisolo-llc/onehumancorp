package orchestration

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SubAgentWorker struct {
	taskQueue queue.TaskQueue
	spawner   SubAgentSpawner
	stopChan  chan struct{}
}

func NewSubAgentWorker(taskQueue queue.TaskQueue, spawner SubAgentSpawner) *SubAgentWorker {
	return &SubAgentWorker{
		taskQueue: taskQueue,
		spawner:   spawner,
		stopChan:  make(chan struct{}),
	}
}

func (w *SubAgentWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(2 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-w.stopChan:
				return
			case <-ticker.C:
				w.poll(ctx)
			}
		}
	}()
}

func (w *SubAgentWorker) poll(ctx context.Context) {
	for {
		// Respect context cancellation during polling
		if ctx.Err() != nil {
			return
		}

		job, err := w.taskQueue.Dequeue(ctx, []string{})
		if err != nil || job == nil {
			return
		}



		go func(job *queue.Job) {
			startTime := time.Now()

			err := w.spawner.SpawnIsolated(ctx, job)

			duration := time.Since(startTime).Seconds()
			telemetry.RecordSubAgentExecutionDuration(ctx, duration)

			if err != nil {
				telemetry.RecordSubAgentFailure(ctx)
				_ = w.taskQueue.Fail(ctx, job.ID, err.Error())
			} else {
				_ = w.taskQueue.Complete(ctx, job.ID)
			}
		}(job)


	}
}

func (w *SubAgentWorker) Stop() {
	close(w.stopChan)
}
