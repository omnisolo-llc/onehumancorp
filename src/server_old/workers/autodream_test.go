package workers

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestAutoDreamWorker_ProcessCompletedTasks(t *testing.T) {
	provider := setupTestDB(t)

	_, err := provider.Exec(context.Background(), `
	CREATE TABLE IF NOT EXISTS tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		auto_dreamed BOOLEAN DEFAULT false
	)`)
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		task_id TEXT,
		raw_content TEXT,
		summary_embedding TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	)`)
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
	INSERT INTO tasks (id, organization_id, payload, status, auto_dreamed)
	VALUES ('task-1', 'org-1', 'Test Payload', 'COMPLETED', false)
	`)
	assert.NoError(t, err)

	worker := NewAutoDreamWorker(provider)
	worker.ProcessCompletedTasks(context.Background())

	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_memories WHERE task_id = 'task-1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	var autoDreamed bool
	err = provider.QueryRow(context.Background(), "SELECT auto_dreamed FROM tasks WHERE id = 'task-1'").Scan(&autoDreamed)
	assert.NoError(t, err)
	assert.True(t, autoDreamed)

	var content string
	err = provider.QueryRow(context.Background(), "SELECT raw_content FROM agent_memories WHERE task_id = 'task-1'").Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Summary of task: Test Payload", content)
}
