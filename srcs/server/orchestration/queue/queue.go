package queue

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// Job represents a background execution unit in the queue.
type Job struct {
	ID           string
	ParentTaskID string
	AgentRole    string
	Payload      string
	Status       string
	Attempts     int
	MaxAttempts  int
	RunAfter     time.Time
	LockedUntil  *time.Time
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

// TaskQueue is an interface for queueing sub-agent tasks securely.
type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}

// NewTaskQueue initializes a TaskQueue based on the environment
func NewTaskQueue(provider db.Provider, redisClient rueidis.Client) TaskQueue {
	if redisClient != nil {
		return &RedisTaskQueue{client: redisClient}
	}
	return &SQLiteTaskQueue{db: provider}
}
