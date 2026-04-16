package pipeline

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockLLM struct {
	response string
}

func (m *mockLLM) Complete(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error) {
	return &local.AssistantMessage{
		Text: m.response,
	}, nil
}

func TestAutoDreamPipeline_Batch(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	// Make sure consolidated_memory table exists
	ctx := context.Background()

	// Insert mock stale session
	oldTime := time.Now().Add(-2 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'test context', ?)", oldTime)
	if err != nil {
		t.Fatalf("failed to insert mock session: %v", err)
	}

	pipeline := NewAutoDreamPipeline(pool.Provider, nil)
	pipeline.llm = &mockLLM{response: "Summarized mock context"}

	pipeline.processBatch(ctx)

	// Verify session was deleted
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count sessions: %v", err)
	}
	if count != 0 {
		t.Errorf("expected session to be deleted, got %d", count)
	}

	// Verify consolidated_memory was inserted
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'session_compression'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count consolidated memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 consolidated memory, got %d", count)
	}
}

func TestAutoDreamPipeline_Files(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	// Create a temporary memory directory and point the pipeline at it.
	memDir := t.TempDir()
	t.Setenv("OHC_MEMORY_DIR", memDir)

	if err := os.WriteFile(filepath.Join(memDir, "test.yml"), []byte("file context"), 0o644); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	pipeline := NewAutoDreamPipeline(pool.Provider, nil)
	pipeline.llm = &mockLLM{response: "Summarized file context"}

	pipeline.processFiles(ctx)

	// Verify consolidated_memory was inserted
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'file_ingestion'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count consolidated memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 consolidated memory, got %d", count)
	}

	// File should be deleted
	if _, err := os.Stat(filepath.Join(memDir, "test.yml")); !os.IsNotExist(err) {
		t.Errorf("expected test file to be deleted")
	}
}
