package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSQLiteTaskQueue(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	defer db.Close()

	q, err := NewSQLiteTaskQueue(db)
	require.NoError(t, err)

	ctx := context.Background()

	// 1. Enqueue
	job := &Job{
		ID:      "job-1",
		Type:    "email",
		Payload: json.RawMessage(`{"to":"test@example.com"}`),
	}
	err = q.Enqueue(ctx, job)
	require.NoError(t, err)

	// 2. Dequeue
	dequeued, err := q.Dequeue(ctx, []string{})
	require.NoError(t, err)
	require.NotNil(t, dequeued)
	assert.Equal(t, "job-1", dequeued.ID)
	assert.Equal(t, "PROCESSING", dequeued.Status)

	// 3. Complete
	err = q.Complete(ctx, "job-1")
	require.NoError(t, err)

	// Verify it's not dequeued again
	dequeued2, err := q.Dequeue(ctx, []string{})
	require.NoError(t, err)
	assert.Nil(t, dequeued2)
}
