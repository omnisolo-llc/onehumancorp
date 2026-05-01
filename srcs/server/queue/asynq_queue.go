package queue

import (
	"context"
	"encoding/json"

	"github.com/hibiken/asynq"
)

const (
	TypeSubAgentJob = "ohc:queue:subagents"
)

type asynqSubAgentPayload struct {
	TaskID  string `json:"task_id"`
	Role    string `json:"role"`
	Payload []byte `json:"payload"`
}

// AsynqQueue implements the Queue interface using Redis and Asynq
type AsynqQueue struct {
	client *asynq.Client
}

// NewAsynqQueue creates a new Redis-backed queue
func NewAsynqQueue(redisOpt asynq.RedisConnOpt) *AsynqQueue {
	return &AsynqQueue{
		client: asynq.NewClient(redisOpt),
	}
}

// EnqueueSubAgent enqueues a sub-agent task
func (q *AsynqQueue) EnqueueSubAgent(ctx context.Context, taskID string, role string, payload []byte) error {
	p := asynqSubAgentPayload{
		TaskID:  taskID,
		Role:    role,
		Payload: payload,
	}

	b, err := json.Marshal(p)
	if err != nil {
		return err
	}

	task := asynq.NewTask(TypeSubAgentJob, b)
	_, err = q.client.EnqueueContext(ctx, task)
	return err
}

// ProcessSubAgentJob is a placeholder for asynq worker implementation,
// usually asynq uses a Server and handlers instead of direct fetching.
// Here we just provide a method that a handler might call.
func (q *AsynqQueue) ProcessSubAgentJob(ctx context.Context, job *Job) error {
	// Processing is handled by asynq.Server and its mux.
	return nil
}
