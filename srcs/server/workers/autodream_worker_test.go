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
    CREATE TABLE IF NOT EXISTS consolidated_memory (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        agent_id TEXT,
        content TEXT NOT NULL,
        embedding TEXT,
        source_type TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )`)
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
	CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING'
	)`)
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
	INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status)
	VALUES ('task-1', 'org-1', 'Test Task', 'Test Description', 'DONE')
	`)
	assert.NoError(t, err)

	worker := NewAutoDreamWorker(provider)
	worker.ProcessCompletedTasks(context.Background())

	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM consolidated_memory WHERE id = 'task-1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	var content string
	err = provider.QueryRow(context.Background(), "SELECT content FROM consolidated_memory WHERE id = 'task-1'").Scan(&content)
	assert.NoError(t, err)
	// Expecting "Task [task-1]: Test Description" based on my change
	assert.Equal(t, "Task [task-1]: Test Description", content)
}
