package pipeline

import (
	"context"
	"encoding/json"
	"os"
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

	// Enqueue and then process via the worker
	pipeline.enqueueStaleSessions(ctx)

	// Wait a moment for queue to process if this were real async, but here we can just invoke logic manually
	// or call the worker logic if it's sync in test

	task, _ := pipeline.queue.Dequeue(ctx, []string{"autodream_pruning"})
	if task != nil {
		var payload map[string]interface{}
		if err := json.Unmarshal([]byte(task.Payload), &payload); err == nil {
			sessionID, _ := payload["session_id"].(string)
			agentID, _ := payload["agent_id"].(string)
			contextData, _ := payload["context_data"].(string)
			pipeline.processSingleSession(ctx, sessionID, agentID, contextData)
		}
	}

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

	// Create test memory directory
	os.MkdirAll(".agent-task/memory", 0755)
	defer os.RemoveAll(".agent-task")

	os.WriteFile(".agent-task/memory/test.yml", []byte("file context"), 0644)

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
	if _, err := os.Stat(".agent-task/memory/test.yml"); !os.IsNotExist(err) {
		t.Errorf("expected test file to be deleted")
	}
}


func TestAutoDreamPipeline_ResolveConflicts(t *testing.T) {
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
	pipeline := NewAutoDreamPipeline(pool.Provider, nil)
	pipeline.llm = &mockLLM{response: "Summarized conflict"}

	// Note: We can't easily mock pgvector<=> behavior in SQLite for conflict detection,
	// but we can call it to ensure it exits gracefully.
	pipeline.resolveConflicts(ctx)
}

func TestAutoDreamPipeline_Start(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())

	pipeline := NewAutoDreamPipeline(pool.Provider, nil)

	// Fast cancel to prevent long running loops
	go func() {
		time.Sleep(100 * time.Millisecond)
		cancel()
	}()

	pipeline.Start(ctx)
}

func TestAutoDreamPipeline_TruthInjectionAndSearch(t *testing.T) {
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
	pipeline := NewAutoDreamPipeline(pool.Provider, nil)

	// Inject Truth
	err = pipeline.InjectTruth(ctx, "truth-1", "AutoDream is an awesome memory system", "[0.1, 0.2, 0.3]")
	if err != nil {
		t.Fatalf("failed to inject truth: %v", err)
	}

	// Verify Injection
	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE id = 'truth-1'").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 injected truth, got %d", count)
	}

	// Search Truth
	results, err := pipeline.SearchTruth(ctx, "[0.1, 0.2, 0.3]", 5)
	if err != nil {
		t.Fatalf("failed to search truth: %v", err)
	}

	if len(results) == 0 {
		t.Fatalf("expected at least 1 search result, got 0")
	}

	if results[0].MemoryID != "truth-1" {
		t.Errorf("expected MemoryID truth-1, got %s", results[0].MemoryID)
	}
}
