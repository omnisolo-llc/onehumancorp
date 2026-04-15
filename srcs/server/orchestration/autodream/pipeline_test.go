package autodream

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	_ "modernc.org/sqlite"

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
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess-1', 'agent-1', 'some context')`)
	if err != nil {
		_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			last_accessed TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);`)
		if err != nil {
			t.Fatalf("failed to create agent_session_data table: %v", err)
		}
		_, err = provider.Exec(ctx, `INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess-1', 'agent-1', 'some context')`)
		if err != nil {
			t.Fatalf("failed to insert data: %v", err)
		}
	}

	tmpDir := t.TempDir()

	err = os.WriteFile(filepath.Join(tmpDir, "dummy.yml"), []byte("insight: test memory context"), 0644)
	if err != nil {
		t.Fatalf("failed to write dummy yaml file: %v", err)
	}

	repo := &mockVectorRepo{}
	llm := &mockLLM{}

	pipeline := NewAutoDreamPipeline(provider, llm, repo, tmpDir)

	err = pipeline.RunConsolidationCycle()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	if len(repo.inserted) != 2 {
		t.Errorf("expected 2 insertions, got %d", len(repo.inserted))
	}

	// Verify cleanup
	rows, err := provider.Query(ctx, `SELECT session_id FROM agent_session_data`)
	if err != nil {
		t.Fatalf("failed to query agent_session_data: %v", err)
	}
	defer rows.Close()
	if rows.Next() {
		t.Errorf("expected agent_session_data to be empty after consolidation cycle")
	}

	files, err := filepath.Glob(filepath.Join(tmpDir, "*.yml"))
	if err != nil {
		t.Fatalf("failed to glob temp dir: %v", err)
	}
	if len(files) > 0 {
		t.Errorf("expected memory files to be deleted after consolidation cycle, found %d", len(files))
	}
}
