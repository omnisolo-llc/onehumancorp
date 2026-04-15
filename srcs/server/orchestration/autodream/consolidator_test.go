package autodream

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockEmbedder struct {
	summary string
	embed   []float32
	err     error
}

func (m *mockEmbedder) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return m.embed, m.err
}

func (m *mockEmbedder) GenerateSummary(ctx context.Context, payload string, logs string) (string, error) {
	return m.summary, m.err
}

func TestConsolidatorProcessCompletedTask(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	// We'll create a simple table that matches the postgres schema but for sqlite to pass the test without errors.
	_, err = sqlDB.Exec(`
		CREATE TABLE shared_tasks_decomposition (
			id TEXT PRIMARY KEY
		);
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			task_id TEXT REFERENCES shared_tasks_decomposition(id),
			content TEXT NOT NULL,
			embedding TEXT,
			metadata TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
		INSERT INTO shared_tasks_decomposition (id) VALUES ('task-123');
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	embedder := &mockEmbedder{
		summary: "Mock summary",
		embed:   []float32{0.1, 0.2, 0.3},
	}
	consolidator := NewConsolidator(provider, embedder)

	task := CompletedTask{
		ID:             "task-123",
		OrganizationID: "org-1",
		Payload:        "payload",
		Logs:           "logs",
	}

	err = consolidator.ProcessCompletedTask(context.Background(), task)
	if err != nil {
		t.Fatalf("ProcessCompletedTask failed: %v", err)
	}

	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 memory, got %d", count)
	}
}
