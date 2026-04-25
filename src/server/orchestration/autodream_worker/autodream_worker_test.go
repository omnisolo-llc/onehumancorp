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
		);
		CREATE TABLE IF NOT EXISTS agent_session_data (session_id TEXT PRIMARY KEY, agent_id TEXT, context_data TEXT);
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem1', 'Test memory 1', NULL)")
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem2', 'Test memory 2', NULL)")
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, context_data) VALUES ('mem1', 'Test memory 1')")
	assert.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, context_data) VALUES ('mem2', 'Test memory 2')")
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
