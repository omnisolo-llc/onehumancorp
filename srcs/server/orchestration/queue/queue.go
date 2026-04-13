package queue

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// Job represents a background execution task for sub-agents.
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

// TaskQueue defines the contract for an execution queue.
type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}

// QueueManager manages sub-agent jobs using either SQLite or PostgreSQL.
type QueueManager struct {
	provider db.Provider
	pgQueue  *PostgresTaskQueue
	sqQueue  *SQLiteTaskQueue
}

// NewQueueManager creates a new QueueManager depending on the db provider.
func NewQueueManager(provider db.Provider) *QueueManager {
	return &QueueManager{
		provider: provider,
		pgQueue:  NewPostgresTaskQueue(provider),
		sqQueue:  NewSQLiteTaskQueue(provider),
	}
}

// Enqueue adds a job to the queue.
func (m *QueueManager) Enqueue(ctx context.Context, job *Job) error {
	if m.provider.IsSQLite() {
		return m.sqQueue.Enqueue(ctx, job)
	}
	return m.pgQueue.Enqueue(ctx, job)
}

// Poll fetches a pending job from the queue.
func (m *QueueManager) Poll(ctx context.Context, roles []string) (*Job, error) {
	if m.provider.IsSQLite() {
		return m.sqQueue.Dequeue(ctx, roles)
	}
	return m.pgQueue.Dequeue(ctx, roles)
}

// Complete marks a job as completed.
func (m *QueueManager) Complete(ctx context.Context, jobID string) error {
	if m.provider.IsSQLite() {
		return m.sqQueue.Complete(ctx, jobID)
	}
	return m.pgQueue.Complete(ctx, jobID)
}

// Fail marks a job as failed, potentially requeuing it based on max attempts.
func (m *QueueManager) Fail(ctx context.Context, jobID string, reason string) error {
	if m.provider.IsSQLite() {
		return m.sqQueue.Fail(ctx, jobID, reason)
	}
	return m.pgQueue.Fail(ctx, jobID, reason)
}
