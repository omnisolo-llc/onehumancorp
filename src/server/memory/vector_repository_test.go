package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestVectorRepository_SemanticSearch(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	// Create table for tests
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories_master (
		id VARCHAR PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		memory_type TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding BLOB,
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		source_task_id VARCHAR
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewVectorRepository(provider)

	record1 := &EmbeddingRecord{
		ID:             "mem-1",
		OrganizationID: "test-org",
		MemoryType:     "TEST",
		Content:        "Vector 1",
		Embedding:      []float32{1.0, 0.0},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-1",
	}

	record2 := &EmbeddingRecord{
		ID:             "mem-2",
		OrganizationID: "test-org",
		MemoryType:     "TEST",
		Content:        "Vector 2",
		Embedding:      []float32{0.0, 1.0},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-2",
	}

	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("failed to upsert record 1: %v", err)
	}
	if err := repo.Upsert(ctx, record2); err != nil {
		t.Fatalf("failed to upsert record 2: %v", err)
	}

	// Search close to record 1
	results, err := repo.SemanticSearch(ctx, "test-org", []float32{0.9, 0.1}, 1)
	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}

	if len(results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(results))
	}

	if results[0].ID != "mem-1" {
		t.Errorf("expected mem-1 to be nearest, got %s", results[0].ID)
	}
}
