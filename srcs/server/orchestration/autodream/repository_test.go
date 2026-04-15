package autodream

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRepository_InsertAndSearch(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open test sqlite db: %v", err)
	}
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding BLOB,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	repo := NewRepository(provider)

	mem1 := &Memory{
		OrganizationID: "org1",
		ID:        "mem-1",
		TaskID:    "task-1",
		Content:   "test content 1",
		Embedding: []float32{1.0, 0.0, 0.0},
	}
	mem2 := &Memory{
		OrganizationID: "org2",
		ID:        "mem-2",
		TaskID:    "task-2",
		Content:   "test content 2",
		Embedding: []float32{0.0, 1.0, 0.0},
	}

	if err := repo.Insert(ctx, mem1); err != nil {
		t.Fatalf("Insert mem1 failed: %v", err)
	}
	if err := repo.Insert(ctx, mem2); err != nil {
		t.Fatalf("Insert mem2 failed: %v", err)
	}

	// Search for something close to mem2
	results, err := repo.Search(ctx, []float32{0.1, 0.9, 0.0}, 1)
	if err != nil {
		t.Fatalf("Search failed: %v", err)
	}

	if len(results) != 1 {
		t.Fatalf("Expected 1 result, got %d", len(results))
	}

	if results[0].ID != "mem-2" {
		t.Errorf("Expected ID mem-2, got %s", results[0].ID)
	}
	if results[0].Content != "test content 2" {
		t.Errorf("Expected Content 'test content 2', got '%s'", results[0].Content)
	}
}
