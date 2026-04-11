package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

type mockEmbeddingClient struct{}

func (m *mockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamWorker_Process(t *testing.T) {
	f, err := os.CreateTemp("", "testdb-*.sqlite")
	require.NoError(t, err)
	defer os.Remove(f.Name())

	provider, err := db.NewSQLiteProvider(f.Name())
	require.NoError(t, err)

	ctx := context.Background()

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

	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, title, status, organization_id, agent_id, payload)
		VALUES ('task-1', 'Test Task', 'COMPLETED', 'org-1', 'agent-1', '{"result": "success"}')
	`)
	require.NoError(t, err)

	worker := NewAutoDreamWorker(provider)
	worker.client = &mockEmbeddingClient{}
	worker.process(ctx)

	rows, err := provider.Query(ctx, "SELECT id, content, source_type, embedding FROM autodream_memories")
	require.NoError(t, err)
	defer rows.Close()

	var count int
	for rows.Next() {
		count++
	}
	assert.Equal(t, 1, count)
}

func TestAutoDreamWorker_StartStop(t *testing.T) {
	f, err := os.CreateTemp("", "testdb-*.sqlite")
	require.NoError(t, err)
	defer os.Remove(f.Name())

	provider, err := db.NewSQLiteProvider(f.Name())
	require.NoError(t, err)
	worker := NewAutoDreamWorker(provider)

	go worker.Start(context.Background())
	time.Sleep(50 * time.Millisecond)
	worker.Stop()
}
