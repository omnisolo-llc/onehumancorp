package queue

import (
	"database/sql"
	_ "modernc.org/sqlite"
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSQLiteTaskQueue_EnqueueDequeue(t *testing.T) {
	d, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	provider := db.NewSqliteProvider(d)
	defer provider.Close()

	q := NewSQLiteTaskQueue(provider)
	ctx := context.Background()

	job := &Job{
		ID:           "job-1",
		ParentTaskID: "task-1",
		AgentRole:    "swe",
		Payload:      `{"task": "code"}`,
		MaxAttempts:  3,
	}

	err = q.Enqueue(ctx, job)
	require.NoError(t, err)

	// Dequeue
	dequeued, err := q.Dequeue(ctx, []string{"swe"})
	require.NoError(t, err)
	require.NotNil(t, dequeued)
	assert.Equal(t, "job-1", dequeued.ID)
	assert.Equal(t, 1, dequeued.Attempts) // Increment on dequeue

	// Dequeue empty
	empty, err := q.Dequeue(ctx, []string{"swe"})
	require.NoError(t, err)
	assert.Nil(t, empty)

	// Complete
	err = q.Complete(ctx, "job-1")
	require.NoError(t, err)

	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_jobs WHERE id = 'job-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

func TestSQLiteTaskQueue_Fail(t *testing.T) {
	d, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	provider := db.NewSqliteProvider(d)
	defer provider.Close()

	q := NewSQLiteTaskQueue(provider)
	ctx := context.Background()

	job := &Job{
		ID:           "job-fail-1",
		ParentTaskID: "task-1",
		AgentRole:    "swe",
		Payload:      `{"task": "fail"}`,
		MaxAttempts:  2,
	}

	err = q.Enqueue(ctx, job)
	require.NoError(t, err)

	// Dequeue 1
	dequeued, err := q.Dequeue(ctx, []string{"swe"})
	require.NoError(t, err)
	require.NotNil(t, dequeued)

	// Fail 1
	err = q.Fail(ctx, dequeued.ID, "some error")
	require.NoError(t, err)

	// Verify status is QUEUED (since attempts < max)
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_jobs WHERE id = 'job-fail-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "QUEUED", status)

	// We need to bypass the run_after delay for test
	_, _ = provider.Exec(ctx, "UPDATE sub_agent_jobs SET run_after = CURRENT_TIMESTAMP WHERE id = 'job-fail-1'")

	// Dequeue 2
	dequeued2, err := q.Dequeue(ctx, []string{"swe"})
	require.NoError(t, err)
	require.NotNil(t, dequeued2)

	// Fail 2
	err = q.Fail(ctx, dequeued2.ID, "some error again")
	require.NoError(t, err)

	// Verify status is FAILED (since attempts >= max)
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_jobs WHERE id = 'job-fail-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "FAILED", status)
}
