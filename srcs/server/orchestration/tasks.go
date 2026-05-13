package orchestration

import (
	"context"

	"onehumancorp/srcs/server/orchestration/queue"
)

type TaskManager struct {
	queue queue.TaskQueue
}

func NewTaskManager(q queue.TaskQueue) *TaskManager {
	return &TaskManager{
		queue: q,
	}
}

func (tm *TaskManager) DelegateSubTask(ctx context.Context, job *queue.Job) error {
	return tm.queue.Enqueue(ctx, job)
}
