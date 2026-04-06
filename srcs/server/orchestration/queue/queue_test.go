package queue

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func TestSQLiteTaskQueue_EnqueueAndDequeue(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := &SQLiteTaskQueue{db: provider}
	ctx := context.Background()

	// Enqueue
	job := &Job{
		ParentTaskID: "task-123",
		AgentRole:    "researcher",
		Payload:      `{"key": "value"}`,
	}

	err := q.Enqueue(ctx, job)
	assert.NoError(t, err)

	// Verify it got an ID
	assert.NotEmpty(t, job.ID)

	// Dequeue
	dequeuedJob, err := q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.NotNil(t, dequeuedJob)

	assert.Equal(t, job.ID, dequeuedJob.ID)
	assert.Equal(t, "RUNNING", dequeuedJob.Status)
	assert.Equal(t, 1, dequeuedJob.Attempts)

	// Complete
	err = q.Complete(ctx, job.ID)
	assert.NoError(t, err)
}

func TestSQLiteTaskQueue_DequeueEmpty(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := &SQLiteTaskQueue{db: provider}
	ctx := context.Background()

	// Dequeue
	dequeuedJob, err := q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.Nil(t, dequeuedJob) // No jobs
}

func TestSQLiteTaskQueue_FailRetries(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := &SQLiteTaskQueue{db: provider}
	ctx := context.Background()

	job := &Job{
		AgentRole: "coder",
		Payload:   "{}",
	}

	err := q.Enqueue(ctx, job)
	assert.NoError(t, err)

	// Dequeue
	dequeued, err := q.Dequeue(ctx, []string{"coder"})
	assert.NoError(t, err)
	assert.Equal(t, 1, dequeued.Attempts)

	// Fail
	err = q.Fail(ctx, dequeued.ID, "some error")
	assert.NoError(t, err)

	// Needs to wait before we can dequeue again due to the +1 minute run_after
	// So dequeue immediately should yield nil
	// Or we just verify the status using db provider directly.
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_jobs WHERE id = $1", dequeued.ID).Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "QUEUED", status)
}
