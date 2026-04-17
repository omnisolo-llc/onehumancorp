package kairos

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

type mockLLM struct{}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupTestDB(t *testing.T) db.Provider {
	provider := db.NewTestProvider(t)
	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			status TEXT,
			payload TEXT
		)
	`)
	assert.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_kairos (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			task_id TEXT,
			content TEXT,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	return provider
}

func TestAutoDreamWorker_ProcessCompletedTasks(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	defer provider.Close()

	// Insert a completed task
	taskID := "task-1"
	orgID := "org-1"
	payload := "Some task payload"
	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES (?, ?, 'DONE', ?)", taskID, orgID, payload)
	assert.NoError(t, err)

	// Insert an incomplete task
	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES (?, ?, 'PENDING', ?)", "task-2", orgID, "Not ready")
	assert.NoError(t, err)

	llm := &mockLLM{}
	worker := NewAutoDreamWorker(provider, llm, 0)

	err = worker.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	// Verify the completed task was embedded and saved
	var savedContent, savedEmbedding string
	err = provider.QueryRow(ctx, "SELECT content, embedding FROM autodream_kairos WHERE task_id = ?", taskID).Scan(&savedContent, &savedEmbedding)
	assert.NoError(t, err)
	assert.Equal(t, payload, savedContent)
	assert.Contains(t, savedEmbedding, "0.1")

	// Verify the pending task was ignored
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_kairos WHERE task_id = ?", "task-2").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 0, count)

	// Test duplicate avoidance
	err = worker.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_kairos").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}

func TestAutoDreamWorker_Start(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	llm := &mockLLM{}
	worker := NewAutoDreamWorker(provider, llm, 50*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)

	_, _ = provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES (?, ?, 'DONE', ?)", "task-3", "org", "content")

	time.Sleep(100 * time.Millisecond)
	cancel()

	var count int
	err := provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM autodream_kairos").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}
