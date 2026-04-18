package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockEmbeddingClient struct{}

func (m *mockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamPipeline_Process(t *testing.T) {
	provider, err := db.NewSQLiteProvider(":memory:")
	require.NoError(t, err)

	ctx := context.Background()

	// Initialize tables
	_, err = provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL,
			priority TEXT,
			agent_id TEXT,
			organization_id TEXT NOT NULL,
			payload TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT 0
		);
	`)
	require.NoError(t, err)

	_, err = provider.Exec(ctx, `
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'test context', datetime('now', '-2 hours'))")
	require.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider)

	// Mock the LLM client
	pipeline.client = &mockEmbeddingClient{}

	// Run process
	pipeline.process(ctx)

	// Verify DB sessions were consolidated
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'session_compression'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 1, count)

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's1'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 0, count)
}

func TestAutoDreamPipeline_StartStop(t *testing.T) {
	provider, err := db.NewSQLiteProvider(":memory:")
	require.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider)

	go pipeline.Start(context.Background())

	// Just checking that we can start and stop it without deadlocking
	time.Sleep(50 * time.Millisecond)
	pipeline.Stop()
}
