package autodream

import (
	"context"
    "database/sql"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
    _ "modernc.org/sqlite"
)

type mockLLMClient struct{}

func (m *mockLLMClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "summarized content", nil
}

func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	vec := make([]float32, 1536)
	vec[0] = 0.5
	vec[1] = 0.5
	return vec, nil
}

func TestAutoDreamPipeline(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
    require.NoError(t, err)
    provider := db.NewSqliteProvider(dbConn)

	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT,
			context_data TEXT
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

	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess1', 'agent1', 'raw data')")
	require.NoError(t, err)

	tempDir := t.TempDir()
	os.Setenv("OHC_MEMORY_DIR", tempDir)
	defer os.Unsetenv("OHC_MEMORY_DIR")

	err = os.WriteFile(filepath.Join(tempDir, "memory1.yml"), []byte("file content"), 0644)
	require.NoError(t, err)

	llm := &mockLLMClient{}
	pipeline := NewAutoDreamPipeline(provider, llm, nil)

	pipeline.process(ctx)

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 2, count)

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 0, count)

	files, _ := os.ReadDir(tempDir)
	assert.Equal(t, 0, len(files))
}

func TestAutoDreamPipelineStartStop(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
    require.NoError(t, err)
    provider := db.NewSqliteProvider(dbConn)

	llm := &mockLLMClient{}
	pipeline := NewAutoDreamPipeline(provider, llm, nil)

	ctx, cancel := context.WithCancel(context.Background())
	go pipeline.Start(ctx)

	time.Sleep(50 * time.Millisecond)
	cancel()
	time.Sleep(50 * time.Millisecond)
}
