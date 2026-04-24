package orchestration

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockLLMClient struct {
	embeddings [][]float32
}

func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	emb := make([]float32, 1536)
	emb[0] = 0.5
	m.embeddings = append(m.embeddings, emb)
	return emb, nil
}

func TestAutoDreamWorkerPipeline(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	_, err := provider.Exec(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
	if err != nil {
		t.Logf("Vector extension error (expected in SQLite): %v", err)
	}

	// Create table
	var createTableQuery string
	if provider.IsSQLite() {
		createTableQuery = `CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			tenant_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)`
	} else {
		createTableQuery = `CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			tenant_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding VECTOR(1536),
			source_type TEXT NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)`
	}

	_, err = provider.Exec(ctx, createTableQuery)
	if err != nil {
		t.Fatalf("failed to create consolidated_memory: %v", err)
	}

	// Create test fs data in a temporary directory
	tempDir := t.TempDir()
	os.Setenv("AGENT_TASK_MEMORY_DIR", tempDir)
	defer os.Unsetenv("AGENT_TASK_MEMORY_DIR")

	err = os.WriteFile(filepath.Join(tempDir, "test.yml"), []byte("test fs memory content"), 0644)
	if err != nil {
		t.Fatalf("failed to write test fs data: %v", err)
	}

	llm := &mockLLMClient{}
	worker := NewAutoDreamWorkerPipeline(provider, llm)

	err = worker.Run(ctx)
	if err != nil {
		t.Fatalf("Run failed: %v", err)
	}

	// Verify file was deleted
	files, err := os.ReadDir(tempDir)
	if err != nil {
		t.Fatalf("failed to read temp dir: %v", err)
	}
	if len(files) != 0 {
		t.Fatalf("expected 0 files in temp dir, got %d", len(files))
	}

	// Verify insertions
	var count int
	err = provider.QueryRow(ctx, `SELECT COUNT(*) FROM consolidated_memory`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to count consolidated_memory: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 memory, got %d", count)
	}

	// Verify NN matching
	if !provider.IsSQLite() {
		testEmb := make([]float32, 1536)
		testEmb[0] = 0.5
		embBytes, _ := json.Marshal(testEmb)
		embStr := string(embBytes)

		query := `SELECT id FROM consolidated_memory ORDER BY embedding <-> $1::vector LIMIT 1`
		var id string
		err = provider.QueryRow(ctx, query, embStr).Scan(&id)
		if err != nil {
			t.Fatalf("nearest neighbor query failed: %v", err)
		}
		if id == "" {
			t.Fatalf("expected valid id from NN query")
		}
	}

	if len(llm.embeddings) != 1 {
		t.Fatalf("expected 1 LLM embedding generation calls, got %d", len(llm.embeddings))
	}
}
