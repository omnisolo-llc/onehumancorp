package orchestration

import (
	"context"
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

		job, err := w.taskQueue.Acquire(ctx, []string{})
		if err != nil || job == nil {
			return
		}



		go func(job *queue.Job) {
			startTime := time.Now()

			ac := &AgentContext{
				AgentID:         job.ID,
				AgentType:       job.AgentRole,
				ParentSessionID: job.ParentTaskID,

				Env: map[string]string{
					"HTTP_PROXY": "http://127.0.0.1:8080",
					"HTTPS_PROXY": "http://127.0.0.1:8080",
				},
				// Extract tier from payload or set default. Harness gateway routes based on this.
				Tier: "free", // Defaulting to free (ServerlessBackend) to lower idle agent cost
			}

			// Dynamically extract tier from job.Payload if available
			var payloadData map[string]interface{}
			if err := json.Unmarshal([]byte(job.Payload), &payloadData); err == nil {
				if tierVal, ok := payloadData["tier"].(string); ok && tierVal != "" {
					ac.Tier = tierVal
				}
			}

			agentCtx := WithAgentContext(ctx, ac)
			err := w.spawner.SpawnIsolated(agentCtx, job)

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
