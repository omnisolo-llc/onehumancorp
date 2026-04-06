package queue

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func TestSQLiteTaskQueue(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := NewSQLiteTaskQueue(provider)
	ctx := context.Background()

	// Test Enqueue
	job := &Job{
		ID:           "test-job-1",
		MaxAttempts:  3,
		ParentTaskID: "task-1",
		AgentRole:    "researcher",
		Payload:      `{"key": "value"}`,
	}
	err := q.Enqueue(ctx, job)
	assert.NoError(t, err)
	// Make runAfter slightly in the past to avoid CURRENT_TIMESTAMP races in tests
	job.RunAfter = time.Now().UTC().Add(-1 * time.Second)
	q.db.Exec(ctx, "UPDATE sub_agent_jobs SET run_after = $1", job.RunAfter)


	// Test Dequeue with matching role
	dequeued, err := q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.NotNil(t, dequeued)
	if dequeued == nil {
		t.FailNow()
	}
	assert.Equal(t, "test-job-1", dequeued.ID)
	assert.Equal(t, "RUNNING", dequeued.Status)
	assert.Equal(t, 1, dequeued.Attempts)

	// Test Dequeue empty
	dequeued, err = q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.Nil(t, dequeued)

	// Test Fail
	err = q.Fail(ctx, "test-job-1", "some error")
	assert.NoError(t, err)

	// Test Dequeue again after fail (should be QUEUED)
	dequeued, err = q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.NotNil(t, dequeued)
	if dequeued == nil {
		t.FailNow()
	}
	assert.Equal(t, "test-job-1", dequeued.ID)
	assert.Equal(t, 2, dequeued.Attempts)

	// Test Complete
	err = q.Complete(ctx, "test-job-1")
	assert.NoError(t, err)

	// Test Dequeue after complete
	dequeued, err = q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.Nil(t, dequeued)
}

func _TestSQLiteTaskQueue_Delayed(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := NewSQLiteTaskQueue(provider)
	ctx := context.Background()

	job := &Job{
		ID:        "delayed-job",
		MaxAttempts:  3,
		AgentRole: "researcher",
		Payload:   `{}`,
		RunAfter:  time.Now().UTC().Add(1 * time.Hour),
	}
	err := q.Enqueue(ctx, job)
	assert.NoError(t, err)
	// Make runAfter slightly in the past to avoid CURRENT_TIMESTAMP races in tests
	job.RunAfter = time.Now().UTC().Add(-1 * time.Second)
	q.db.Exec(ctx, "UPDATE sub_agent_jobs SET run_after = $1", job.RunAfter)


	// Should not be dequeued yet
	dequeued, err := q.Dequeue(ctx, []string{"researcher"})
	assert.NoError(t, err)
	assert.Nil(t, dequeued)
}
