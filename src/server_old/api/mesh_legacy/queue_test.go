package mesh

import (
	"context"
	"database/sql"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	uri := fmt.Sprintf("file:%s?mode=memory&cache=shared", t.Name())
	conn, err := sql.Open("sqlite", uri)
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = conn.Exec(`
		CREATE TABLE mission_queue (
			mission_id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			assigned_agent TEXT,
			priority TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE sub_agent_queue (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	return conn
}

func TestEnqueueMission_NilDB(t *testing.T) {
	ctx := context.Background()
	_, err := EnqueueMission(ctx, nil, "test", "P0", []byte("{}"))
	assert.Error(t, err)
}

func TestCompleteMission_NilDB(t *testing.T) {
	ctx := context.Background()
	err := CompleteMission(ctx, nil, "uuid", "agent")
	assert.Error(t, err)
}

func TestQueueOrchestrator_NilDB(t *testing.T) {
	ctx := context.Background()
	q := NewQueueOrchestrator(nil, nil, false)

	_, err := q.EnqueueSubTask(ctx, "parent", []byte("{}"))
	assert.Error(t, err)

	_, err = q.ClaimSubTask(ctx, "worker")
	assert.Error(t, err)

	err = q.CompleteSubTask(ctx, "task", "worker")
	assert.Error(t, err)
}

func TestQueueOrchestrator_Flow(t *testing.T) {
	conn := setupTestDB(t)
	defer conn.Close()

	ctx := context.Background()
	q := NewQueueOrchestrator(conn, nil, true)

	// 1. Enqueue
	taskID, err := q.EnqueueSubTask(ctx, "parent-1", []byte(`{"do":"work"}`))
	assert.NoError(t, err)
	assert.NotEmpty(t, taskID)

	// 2. Claim
	task, err := q.ClaimSubTask(ctx, "worker-1")
	assert.NoError(t, err)
	assert.NotNil(t, task)
	assert.Equal(t, taskID, task.ID)
	assert.Equal(t, "parent-1", task.ParentTaskID)
	assert.Equal(t, "IN_PROGRESS", task.Status)
	assert.NotNil(t, task.WorkerID)
	assert.Equal(t, "worker-1", *task.WorkerID)

	// 3. Claim again should return nil (no tasks)
	task2, err := q.ClaimSubTask(ctx, "worker-2")
	assert.NoError(t, err)
	assert.Nil(t, task2)

	// 4. Complete
	err = q.CompleteSubTask(ctx, taskID, "worker-1")
	assert.NoError(t, err)

	// 5. Verify completed
	var status string
	err = conn.QueryRow("SELECT status FROM sub_agent_queue WHERE id = $1", taskID).Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}
