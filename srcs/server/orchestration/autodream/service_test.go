package autodream

import (
	"context"
	"database/sql"
	"testing"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamConsolidationPipeline(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		task_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		status TEXT NOT NULL,
		payload TEXT
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES ('task-123', 'org1', 'DONE', 'payload data')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	repo := NewRepository(provider)
	llm := &mockLLM{}
	service := NewConsolidator(repo, llm, provider)

	err = service.ProcessCompletedTasks(ctx)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}
