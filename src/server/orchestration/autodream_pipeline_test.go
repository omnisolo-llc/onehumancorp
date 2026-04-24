package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockEmbeddingClient struct{}

func (m *mockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	vec := make([]float32, 1536)
	vec[0] = 0.1
	vec[1] = 0.2
	vec[2] = 0.3
	return vec, nil
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
		CREATE TABLE IF NOT EXISTS consolidated_memory (
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
		CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
			id TEXT PRIMARY KEY,
			topic TEXT NOT NULL,
			summary TEXT NOT NULL,
			embedding TEXT,
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

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_long_term_memory WHERE topic = 'Session Compression: s1'").Scan(&count)
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

func TestChunkText(t *testing.T) {
	tests := []struct {
		name      string
		text      string
		chunkSize int
		want      []string
	}{
		{
			name:      "empty text",
			text:      "",
			chunkSize: 10,
			want:      []string{},
		},
		{
			name:      "chunk size 0",
			text:      "hello",
			chunkSize: 0,
			want:      []string{"hello"},
		},
		{
			name:      "chunk size negative",
			text:      "hello",
			chunkSize: -1,
			want:      []string{"hello"},
		},
		{
			name:      "text shorter than chunk size",
			text:      "hello",
			chunkSize: 10,
			want:      []string{"hello"},
		},
		{
			name:      "text exactly chunk size",
			text:      "hello",
			chunkSize: 5,
			want:      []string{"hello"},
		},
		{
			name:      "text longer than chunk size",
			text:      "hello world",
			chunkSize: 5,
			want:      []string{"hello", " worl", "d"},
		},
		{
			name:      "multi-byte runes",
			text:      "こんにちは世界", // 7 runes
			chunkSize: 3,
			want:      []string{"こんに", "ちは世", "界"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := chunkText(tt.text, tt.chunkSize)
			assert.Equal(t, tt.want, got)
		})
	}
}

func TestAutoDreamPipeline_FilesMultipleChunks(t *testing.T) {
	provider, err := db.NewSQLiteProvider(":memory:")
	require.NoError(t, err)

	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
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
		CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
			id TEXT PRIMARY KEY,
			topic TEXT NOT NULL,
			summary TEXT NOT NULL,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	dir := t.TempDir()
	t.Setenv("OHC_MEMORY_DIR", dir)

	// Create test file
	fileContent := `content: "A very long content that exceeds chunk limit."`
	err = os.WriteFile(filepath.Join(dir, "mission1.yml"), []byte(fileContent), 0644)
	require.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider)
	pipeline.client = &mockEmbeddingClient{}

	// Let's modify the file loop chunk size temporarily using a monkey patch or we just rely on normal size.
	// Since chunk size is hardcoded to 8000, we can just create a >8000 rune file to test it.

	longContent := make([]byte, 9000)
	for i := range longContent {
		longContent[i] = 'a'
	}
	longYaml := "content: |\n  " + string(longContent)
	err = os.WriteFile(filepath.Join(dir, "mission2.yml"), []byte(longYaml), 0644)
	require.NoError(t, err)

	pipeline.process(ctx)

	// Check if chunks are stored
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory").Scan(&count)
	require.NoError(t, err)
	// mission1 => 1 chunk (content is < 8000)
	// mission2 => 2 chunks (content is > 8000)
	assert.Equal(t, 3, count)

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_long_term_memory").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 3, count)
}
