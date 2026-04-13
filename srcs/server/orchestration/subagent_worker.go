package orchestration

import (
	"context"
	"encoding/json"
	"time"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
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
		if ctx.Err() != nil {
			return
		}
		job, err := w.taskQueue.Dequeue(ctx, []string{})
		if err != nil || job == nil {
			return
		}
		go w.processJob(ctx, job)
	}
}

func (w *SubAgentWorker) processJob(ctx context.Context, job *queue.Job) {
	var payload map[string]interface{}
	if err := json.Unmarshal([]byte(job.Payload), &payload); err != nil {
		_ = w.taskQueue.Fail(ctx, job.ID, fmt.Sprintf("invalid payload: %v", err))
		return
	}

	orgID := ""
	if val, ok := payload["organization_id"].(string); ok {
		orgID = val
	}

	task := &SharedTask{
		ID:             job.ParentTaskID,
		OrganizationID: orgID,
		Priority:       "DELEGATED",
	}

	if err := w.spawner.Spawn(ctx, task); err != nil {
		_ = w.taskQueue.Fail(ctx, job.ID, err.Error())
	} else {
		_ = w.taskQueue.Complete(ctx, job.ID)
	}
}

func (w *SubAgentWorker) Stop() {
	close(w.stopChan)
}
