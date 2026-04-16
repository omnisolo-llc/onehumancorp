package workers

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func TestAutoDreamDataPipeline_Extraction(t *testing.T) {
	provider := db.NewTestProvider(t)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Setup schema for testing
	_, err := provider.Exec(ctx, `
		CREATE TABLE shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT
		);
		CREATE TABLE knowledge_embeddings (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT
		);
	`)
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, `INSERT INTO shared_tasks_decomposition (id, status, payload) VALUES ('task-1', 'DONE', 'Test plan payload')`)
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, `INSERT INTO shared_tasks_decomposition (id, status, payload) VALUES ('task-2', 'PENDING', 'Pending plan payload')`)
	assert.NoError(t, err)

	pipeline := NewAutoDreamDataPipeline(provider)
	pipeline.RunPipeline(ctx)

	// Verify embedding inserted
	var count int
	err = provider.QueryRow(ctx, `SELECT COUNT(*) FROM knowledge_embeddings`).Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count, "Should have extracted exactly 1 completed task")

	var content string
	err = provider.QueryRow(ctx, `SELECT content FROM knowledge_embeddings LIMIT 1`).Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Test plan payload", content)
}
