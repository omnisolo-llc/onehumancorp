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
		CREATE TABLE autodream_memories (
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
		INSERT INTO shared_tasks (id, title, status, organization_id, agent_id, payload)
		VALUES ('task-1', 'Test Task', 'COMPLETED', 'org-1', 'agent-1', '{"result": "success"}')
	`)
	require.NoError(t, err)

	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, title, status, organization_id, agent_id, payload)
		VALUES ('task-2', 'Pending Task', 'PENDING', 'org-1', 'agent-1', '{"result": "waiting"}')
	`)
	require.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider)

	// Mock the LLM client
	pipeline.client = &mockEmbeddingClient{}

	// Run process
	pipeline.process(ctx)

	// Verify only completed tasks were consolidated
	rows, err := provider.Query(ctx, "SELECT id, content, source_type, embedding FROM autodream_memories")
	require.NoError(t, err)
	defer rows.Close()

	var memories []struct {
		id         string
		content    string
		sourceType string
		embedding  string
	}

	for rows.Next() {
		var m struct {
			id         string
			content    string
			sourceType string
			embedding  string
		}
		err := rows.Scan(&m.id, &m.content, &m.sourceType, &m.embedding)
		require.NoError(t, err)
		memories = append(memories, m)
	}

	assert.Len(t, memories, 1)
	assert.Equal(t, "task-1", memories[0].id)
	assert.Equal(t, `{"result": "success"}`, memories[0].content)
	assert.Equal(t, "shared_task", memories[0].sourceType)
	assert.Equal(t, "[0.1,0.2,0.3]", memories[0].embedding)
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
