package autodream

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type mockEmbeddingService struct{}

func (m *mockEmbeddingService) GetEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestConsolidator(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			name TEXT NOT NULL,
			description TEXT,
			output_payload TEXT,
			organization_id TEXT NOT NULL,
			state TEXT NOT NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create mock table: %v", err)
	}

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			metadata TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create mock memories table: %v", err)
	}

	_, err = provider.Exec(context.Background(), `
		INSERT INTO shared_tasks_decomposition (id, name, description, output_payload, organization_id, state)
		VALUES ('task_1', 'Test Task', 'A task to test consolidation', '{"result": "success"}', 'org_1', 'DONE')
	`)
	if err != nil {
		t.Fatalf("Failed to insert mock task: %v", err)
	}

	repo := NewRepository(provider)
	embService := &mockEmbeddingService{}
	consolidator := NewConsolidator(repo, embService)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err = consolidator.ProcessCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("Consolidation failed: %v", err)
	}

	memories, err := repo.Search(ctx, []float32{0.1, 0.2, 0.3}, 10)
	if err != nil {
		t.Fatalf("Failed to search memories: %v", err)
	}

	if len(memories) != 1 {
		t.Fatalf("Expected 1 memory, got %d", len(memories))
	}

	if memories[0].TaskID != "task_1" {
		t.Errorf("Expected TaskID task_1, got %s", memories[0].TaskID)
	}
	if memories[0].OrganizationID != "org_1" {
		t.Errorf("Expected OrganizationID org_1, got %s", memories[0].OrganizationID)
	}
}
