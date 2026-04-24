package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestVectorRepository(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

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

	// Test Upsert
	record1 := &EmbeddingRecord{
		ID:             "mem-1",
		OrganizationID: "org-1",
		MemoryType:     "TEST",
		Content:        "test content 1",
		Embedding:      []float32{1.0, 0.0},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-1",
	}
	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("failed to upsert: %v", err)
	}

	record2 := &EmbeddingRecord{
		ID:             "mem-2",
		OrganizationID: "org-1",
		MemoryType:     "TEST",
		Content:        "test content 2",
		Embedding:      []float32{0.0, 1.0},
		CreatedAt:      time.Now().Add(-2 * time.Hour), // Older record for pruning test
		SourceTaskID:   "task-2",
	}
	if err := repo.Upsert(ctx, record2); err != nil {
		t.Fatalf("failed to upsert: %v", err)
	}

	// Test duplicate Upsert
	record1.Content = "updated content 1"
	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("failed to upsert duplicate: %v", err)
	}

	// Test SemanticSearch
	// Querying for vector closest to [0.0, 1.0], which should return record2 then record1
	res, err := repo.SemanticSearch(ctx, "org-1", []float32{0.0, 1.0}, 10)
	if err != nil {
		t.Fatalf("failed semantic search: %v", err)
	}
	if len(res) != 2 {
		t.Fatalf("expected 2 records, got: %v", len(res))
	}
	if res[0].ID != "mem-2" {
		t.Fatalf("expected mem-2 to be the closest record, got: %s", res[0].ID)
	}
	if res[1].ID != "mem-1" {
		t.Fatalf("expected mem-1 to be the second closest record, got: %s", res[1].ID)
	}

	// Test PruneStaleContext
	deleted, err := repo.PruneStaleContext(ctx, "org-1", time.Now().Add(-1*time.Hour))
	if err != nil {
		t.Fatalf("failed to prune: %v", err)
	}
	if deleted != 1 {
		t.Fatalf("expected 1 deleted record, got %d", deleted)
	}
}
