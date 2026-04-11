package orchestration

import (
	"database/sql"
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

func TestAutoDreamPipeline_Process(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "testdb_*.db")
	require.NoError(t, err)
	defer os.Remove(tmpFile.Name())
	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	require.NoError(t, err)
	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
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
		INSERT INTO agent_session_data (session_id, agent_id, context_data, created_at, last_accessed)
		VALUES ('session-1', 'agent-1', 'mock session content', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`)
	require.NoError(t, err)

	worker := NewAutoDreamPipeline(provider)
	worker.client = &mockEmbeddingClient{}
	worker.processSessionData(ctx)

	rows, err := provider.Query(ctx, "SELECT id, content, source_type, embedding FROM autodream_memories")
	require.NoError(t, err)
	defer rows.Close()

	var count int
	for rows.Next() {
		count++
	}
	assert.Equal(t, 1, count)
}

func TestAutoDreamPipeline_StartStop(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "testdb_*.db")
	require.NoError(t, err)
	defer os.Remove(tmpFile.Name())
	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	require.NoError(t, err)
	provider := db.NewSqliteProvider(sqlDB)
	worker := NewAutoDreamPipeline(provider)
	go worker.Start(context.Background())
	time.Sleep(50 * time.Millisecond)
	worker.Stop()
}
