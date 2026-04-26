package autodream_worker

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
)

type mockConsolidatorEmbeddingClient struct {
	calls       int
	shouldError bool
}

func (m *mockConsolidatorEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.calls++
	if m.shouldError {
		return nil, errors.New("mock error")
	}
	if text == "empty" {
		return []float32{}, nil
	}
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

func TestAutoDreamConsolidator_ProcessBacklog_Empty(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			processed_at DATETIME
		)
	`)
	assert.NoError(t, err)

	mockLLM := &mockConsolidatorEmbeddingClient{}
	consolidator := NewAutoDreamConsolidator(provider, nil, mockLLM)
	err = consolidator.ProcessBacklog(ctx)
	assert.NoError(t, err)
	assert.Equal(t, 0, mockLLM.calls)
}

func TestAutoDreamConsolidator_ProcessBacklog_ErrorEmbedding(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			processed_at DATETIME
		)
	`)
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem3', 'Test memory 3', NULL)")
	assert.NoError(t, err)

	mockLLM := &mockConsolidatorEmbeddingClient{shouldError: true}
	consolidator := NewAutoDreamConsolidator(provider, nil, mockLLM)
	err = consolidator.ProcessBacklog(ctx)
	assert.NoError(t, err)

	rows, err := provider.Query(ctx, "SELECT id FROM autodream_memories WHERE processed_at IS NOT NULL")
	assert.NoError(t, err)
	defer rows.Close()
	assert.False(t, rows.Next())
}

func TestAutoDreamConsolidator_ProcessBacklog_EmptyEmbedding(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			processed_at DATETIME
		)
	`)
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, processed_at) VALUES ('mem4', 'empty', NULL)")
	assert.NoError(t, err)

	mockLLM := &mockConsolidatorEmbeddingClient{}
	consolidator := NewAutoDreamConsolidator(provider, nil, mockLLM)
	err = consolidator.ProcessBacklog(ctx)
	assert.NoError(t, err)

	rows, err := provider.Query(ctx, "SELECT id FROM autodream_memories WHERE processed_at IS NOT NULL")
	assert.NoError(t, err)
	defer rows.Close()
	assert.True(t, rows.Next())
}
