package kairos

import (
	"context"
	"testing"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server_old/db"
)

type mockLLMClient struct {
	called bool
}

func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.called = true
	emb := make([]float32, 1536)
	emb[0] = 0.5
	return emb, nil
}

func TestAutoDreamWorker(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		CREATE TABLE shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT DEFAULT 'PENDING',
			payload TEXT,
			auto_dreamed BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to setup test table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream'
		)
	`)
	if err != nil {
		t.Fatalf("failed to setup autodream_memories test table: %v", err)
	}

	taskID := uuid.New().String()
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, payload, auto_dreamed)
		VALUES ($1, 'test-org', 'Test Task', 'COMPLETED', '{"key": "value"}', false)
	`, taskID)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	mockLLM := &mockLLMClient{}
	worker := NewAutoDreamWorker(provider, mockLLM)

	worker.ProcessCompletedTasks(ctx)

	if !mockLLM.called {
		t.Errorf("expected LLM client to be called")
	}

	var autoDreamed bool
	err = provider.QueryRow(ctx, "SELECT auto_dreamed FROM shared_tasks_decomposition WHERE id = $1", taskID).Scan(&autoDreamed)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}
	if !autoDreamed {
		t.Errorf("expected task to be marked as auto_dreamed")
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE task_id = $1", taskID).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory to be inserted, got %d", count)
	}
}
