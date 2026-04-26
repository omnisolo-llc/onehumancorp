package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSqliteQueue_EnqueueDequeue(t *testing.T) {
	provider := db.NewSqliteProvider(db.SetupTestDB(t))
	q := NewSqliteQueue(provider)
	ctx := context.Background()

	// Ensure table created for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			scheduled_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			completed_at TIMESTAMPTZ,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	payload := map[string]interface{}{"job": "work"}

	// Test Enqueue
	id, err := q.Enqueue(ctx, payload, time.Now())
	require.NoError(t, err)
	assert.NotEmpty(t, id)

	// Test Dequeue
	task, err := q.Dequeue(ctx)
	require.NoError(t, err)
	require.NotNil(t, task)
	assert.Equal(t, id, task.ID)
	assert.Equal(t, "work", task.Payload["job"])

	// Empty Queue
	task2, err := q.Dequeue(ctx)
	require.NoError(t, err)
	assert.Nil(t, task2)

	// Test Complete
	err = q.Complete(ctx, id)
	require.NoError(t, err)

	// Check completion status
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_queue WHERE id = $1", id).Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}
