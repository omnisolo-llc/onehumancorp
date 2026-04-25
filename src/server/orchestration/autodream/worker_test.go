package autodream

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

type MockWorkerLLMClient struct {
	embeddings [][]float32
}

func (m *MockWorkerLLMClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "mocked summary", nil
}

func (m *MockWorkerLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	// Mock 1536 dim embedding
	emb := make([]float32, 1536)
	emb[0] = 0.5
	m.embeddings = append(m.embeddings, emb)
	return emb, nil
}

func TestAutoDreamWorker(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	// Create tables
	_, err := provider.Exec(ctx, `CREATE TABLE agent_session_data (
		session_id TEXT PRIMARY KEY,
		agent_id TEXT NOT NULL,
		context_data TEXT NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		last_accessed TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}

	_, err = provider.Exec(ctx, `CREATE TABLE agent_memory_embeddings (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		tenant_id TEXT NOT NULL DEFAULT '',
		agent_id TEXT NOT NULL,
		memory_type TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create agent_memory_embeddings: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess1', 'agent1', 'test db memory')`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Create test fs data
	tempDir, err := os.MkdirTemp("", "ohc_memory")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)
	os.Setenv("OHC_MEMORY_DIR", tempDir)
	defer os.Unsetenv("OHC_MEMORY_DIR")

	err = os.WriteFile(filepath.Join(tempDir, "test.yml"), []byte("test fs memory"), 0644)
	if err != nil {
		t.Fatalf("failed to write test fs data: %v", err)
	}

	llm := &MockWorkerLLMClient{}
	worker := NewAutoDreamWorker(provider, llm)

	err = worker.RunConsolidation(ctx)
	if err != nil {
		t.Fatalf("RunConsolidation failed: %v", err)
	}

	// Verify DB extraction deleted the row
	var count int
	err = provider.QueryRow(ctx, `SELECT COUNT(*) FROM agent_session_data`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to count agent_session_data: %v", err)
	}
	if count != 0 {
		t.Fatalf("expected 0 agent_session_data, got %d", count)
	}

	// Verify FS extraction deleted the file
	files, err := os.ReadDir(tempDir)
	if err != nil {
		t.Fatalf("failed to read temp dir: %v", err)
	}
	if len(files) != 0 {
		t.Fatalf("expected 0 files in temp dir, got %d", len(files))
	}

	// Verify insertions into agent_memory_embeddings
	err = provider.QueryRow(ctx, `SELECT COUNT(*) FROM agent_memory_embeddings`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to count agent_memory_embeddings: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 agent_memory_embeddings, got %d", count)
	}

	// Verify LLM calls
	if len(llm.embeddings) != 2 {
		t.Fatalf("expected 2 LLM embedding generation calls, got %d", len(llm.embeddings))
	}
}
