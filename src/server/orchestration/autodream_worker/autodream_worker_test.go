package autodream_worker

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
)

type mockConsolidatorEmbeddingClient struct {
	calls int
}

func (m *mockConsolidatorEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.calls++
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamConsolidator_ProcessBacklog(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	// Need to initialize schema
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			processed_at DATETIME
		)
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem1', 'Test memory 1', NULL)")
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem2', 'Test memory 2', NULL)")
	assert.NoError(t, err)


	mockLLM := &mockConsolidatorEmbeddingClient{}

	// Testing without redis client, should still work but skip locking logic.
	consolidator := NewAutoDreamConsolidator(provider, nil, mockLLM)

	err = consolidator.ProcessBacklog(ctx)
	assert.NoError(t, err)

	assert.Equal(t, 2, mockLLM.calls)

	// Verify database state
	rows, err := provider.Query(ctx, "SELECT id, processed_at FROM autodream_memories WHERE processed_at IS NOT NULL")
	assert.NoError(t, err)
	defer rows.Close()

	count := 0
	for rows.Next() {
		count++
	}
	assert.Equal(t, 2, count)
}

func TestAutoDreamConsolidator_ProcessCompletedTasks(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	// Initialize schemas
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_master (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			payload TEXT,
			status TEXT NOT NULL
		)
	`)
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding TEXT,
			source_task_id TEXT
		)
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks_master (id, organization_id, title, description, payload, status) VALUES ('task1', 'org1', 'Task 1', 'Desc 1', '{}', 'COMPLETED')")
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks_master (id, organization_id, title, description, payload, status) VALUES ('task2', 'org1', 'Task 2', 'Desc 2', '{}', 'COMPLETED')")
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks_master (id, organization_id, title, description, payload, status) VALUES ('task3', 'org1', 'Task 3', 'Desc 3', '{}', 'PENDING')")
	assert.NoError(t, err)

	mockLLM := &mockConsolidatorEmbeddingClient{}
	consolidator := NewAutoDreamConsolidator(provider, nil, mockLLM)

	err = consolidator.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	// We have 2 COMPLETED tasks
	assert.Equal(t, 2, mockLLM.calls)

	// Verify database state
	rows, err := provider.Query(ctx, "SELECT id FROM autodream_memories_master")
	assert.NoError(t, err)
	defer rows.Close()

	count := 0
	for rows.Next() {
		count++
	}
	assert.Equal(t, 2, count)

	// Second run should skip already processed tasks
	err = consolidator.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)
	assert.Equal(t, 2, mockLLM.calls) // Still 2 calls, none new
}
