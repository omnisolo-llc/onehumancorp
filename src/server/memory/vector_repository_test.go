package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestVectorRepositoryWithSQLite(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
			source_task_id VARCHAR
		);
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	repo := NewVectorRepository(provider)

	err = repo.Upsert(ctx, &EmbeddingRecord{
		ID:             "emb-1",
		OrganizationID: "org-1",
		Embedding:      []float32{0.1, 0.2},
	})
	if err != nil {
		t.Fatalf("Upsert failed: %v", err)
	}

	err = repo.UpsertConsolidatedMemory(ctx, &ConsolidatedMemoryRecord{
		ID:             "con-1",
		OrganizationID: "org-1",
		Embedding:      []float32{0.1, 0.2},
	})
	if err != nil {
		t.Fatalf("UpsertConsolidatedMemory failed: %v", err)
	}

	res, err := repo.SearchConsolidatedMemories(ctx, "org-1", "", []float32{0.1, 0.2}, 5)
	if err != nil {
		t.Fatalf("SearchConsolidatedMemories failed: %v", err)
	}
	if len(res) == 0 {
		t.Fatal("expected results, got 0")
	}

	err = repo.DeleteConsolidatedMemory(ctx, "con-1", "org-1")
	if err != nil {
		t.Fatalf("DeleteConsolidatedMemory failed: %v", err)
	}

	res, err = repo.GetOldMemories(ctx, "org-1", time.Now(), 5)
	if err != nil {
		t.Fatalf("GetOldMemories failed: %v", err)
	}
	if len(res) != 0 {
		t.Fatal("expected 0 old memories")
	}

	_, _ = repo.SemanticSearch(ctx, "org-1", nil, 1)
}
