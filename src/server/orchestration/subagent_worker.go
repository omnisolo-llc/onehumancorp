package orchestration

import (
	"context"
	"time"
	"fmt"

	"github.com/onehumancorp/mono/src/server/orchestration/queue"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/onehumancorp/mono/src/backend/harness"
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

			allowedDomains := []string{"api.openai.com", "api.anthropic.com"} // Just some default allowed domains

			// Use port 0 to let OS assign a random open port
			proxy := harness.NewNetworkProxy(0, allowedDomains, job.ID)
			server, err := proxy.Start(ctx)
			if err != nil {
				telemetry.RecordSubAgentFailure(ctx)
				_ = w.taskQueue.Fail(ctx, job.ID, fmt.Sprintf("failed to start network proxy: %v", err))
				return
			}
			defer server.Close()

			ac := &AgentContext{
				AgentID:         job.ID,
				AgentType:       job.AgentRole,
				ParentSessionID: job.ParentTaskID,

				Env: map[string]string{
					"HTTP_PROXY": fmt.Sprintf("http://127.0.0.1:%d", proxy.Port),
					"HTTPS_PROXY": fmt.Sprintf("http://127.0.0.1:%d", proxy.Port),
				},
			}
			agentCtx := WithAgentContext(ctx, ac)
			err = w.spawner.SpawnIsolated(agentCtx, job)

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
