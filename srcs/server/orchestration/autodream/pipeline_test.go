package autodream

import (
	"context"
	"database/sql"
	_ "modernc.org/sqlite"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockVectorRepo struct {
	inserted []*Memory
}

func (m *mockVectorRepo) Insert(ctx context.Context, mem *Memory) error {
	m.inserted = append(m.inserted, mem)
	return nil
}

func TestAutoDreamPipeline_RunConsolidationCycle(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS agent_session_data (
		session_id TEXT PRIMARY KEY,
		agent_id TEXT NOT NULL,
		context_data TEXT NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		last_accessed TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess-1', 'agent-1', 'db context data 1')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	tempDir := t.TempDir()
	os.Setenv("OHC_MEMORY_DIR", tempDir)
	defer os.Unsetenv("OHC_MEMORY_DIR")

	file1 := filepath.Join(tempDir, "memory1.yml")
	err = os.WriteFile(file1, []byte("fs context data 1"), 0644)
	if err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	repo := &mockVectorRepo{}
	llm := &mockLLM{}
	pipeline := NewAutoDreamPipeline(repo, llm, provider)

	err = pipeline.RunConsolidationCycle(ctx)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if len(repo.inserted) != 2 {
		t.Errorf("expected 2 insertions, got %d", len(repo.inserted))
	}

	if repo.inserted[0].Content != "Mock summary" {
		t.Errorf("expected 'Mock summary', got '%s'", repo.inserted[0].Content)
	}
	if repo.inserted[1].Content != "Mock summary" {
		t.Errorf("expected 'Mock summary', got '%s'", repo.inserted[1].Content)
	}
}
