package autodream

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type mockEmbeddingClient struct {
	t *testing.T
}

func (m *mockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestConsolidator_Consolidate(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqlDB.Close()
	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Apply migrations or create tables directly
	_, err = provider.Exec(ctx, `
		CREATE TABLE shared_tasks_decomposition (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			status VARCHAR NOT NULL,
			payload JSONB
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			metadata JSONB,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload)
		VALUES ('task1', 'org1', 'COMPLETED', '{"result": "success"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	client := &mockEmbeddingClient{t: t}
	consolidator := NewConsolidator(provider, client)

	err = consolidator.Consolidate(ctx)
	if err != nil {
		t.Fatalf("Consolidate failed: %v", err)
	}

	// Verify memory was created
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 memory, got %d", count)
	}
}
