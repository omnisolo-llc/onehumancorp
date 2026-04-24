package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/src/server/harness"
	"github.com/onehumancorp/mono/src/server/orchestration/queue"
	"github.com/onehumancorp/mono/src/server/telemetry"
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

			// Start a local network proxy for this sub-agent execution
			allowedDomains := []string{"googleapis.com", "stripe.com"} // Added defaults for essential API communication
			proxy := harness.NewNetworkProxy(job.ID, allowedDomains)
			if err := proxy.Start(); err != nil {
				// Fallback to no proxy or handle error
			} else {
				defer proxy.Close()
			}

			proxyURL := proxy.URL()
			if proxyURL == "" {
				proxyURL = "http://127.0.0.1:8080" // Fallback if start failed
			}

			ac := &AgentContext{
				AgentID:         job.ID,
				AgentType:       job.AgentRole,
				ParentSessionID: job.ParentTaskID,

				Env: map[string]string{
					"HTTP_PROXY":  proxyURL,
					"HTTPS_PROXY": proxyURL,
				},
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
