package memory

import (
	"context"
	"database/sql"
	"reflect"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	_ "modernc.org/sqlite" // modernc sqlite
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
		MemoryType:     "TASK_SUMMARY",
		Content:        "test content 1",
		Embedding:      []float32{0.1, 0.2},
		CreatedAt:      time.Now().Add(-2 * time.Hour),
		SourceTaskID:   "task-1",
	}
	record2 := &EmbeddingRecord{
		ID:             "mem-2",
		OrganizationID: "org-1",
		MemoryType:     "TASK_SUMMARY",
		Content:        "test content 2",
		Embedding:      []float32{0.3, 0.4},
		CreatedAt:      time.Now(),
		SourceTaskID:   "task-2",
	}

	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if err := repo.Upsert(ctx, record2); err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test SemanticSearch (SQLite fallback)
	results, err := repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2}, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}
	// order should be by created_at DESC
	if results[0].ID != "mem-2" {
		t.Errorf("expected mem-2 to be first, got %s", results[0].ID)
	}
	if !reflect.DeepEqual(results[1].Embedding, []float32{0.1, 0.2}) {
		t.Errorf("expected embedding to be unmarshaled correctly")
	}

	// Test Delete
	if err := repo.Delete(ctx, []string{"mem-1"}); err != nil {
		t.Fatalf("expected no error on delete, got %v", err)
	}
	results, _ = repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2}, 10)
	if len(results) != 1 || results[0].ID != "mem-2" {
		t.Fatalf("expected 1 result (mem-2) after delete, got %v", results)
	}

	// Test Prune
	// record2 was created just now, so it shouldn't be pruned if we prune older than 1 hour ago
	if err := repo.Prune(ctx, "org-1", time.Now().Add(-1*time.Hour)); err != nil {
		t.Fatalf("expected no error on prune, got %v", err)
	}
	results, _ = repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2}, 10)
	if len(results) != 1 {
		t.Fatalf("expected record to remain after prune")
	}

	// prune with future time
	if err := repo.Prune(ctx, "org-1", time.Now().Add(1*time.Hour)); err != nil {
		t.Fatalf("expected no error on prune, got %v", err)
	}
	results, _ = repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2}, 10)
	if len(results) != 0 {
		t.Fatalf("expected no records to remain after prune")
	}
}

func TestFormatFloat32SliceForVector(t *testing.T) {
	if formatFloat32SliceForVector(nil) != "[]" {
		t.Errorf("expected [] for empty slice")
	}
	res := formatFloat32SliceForVector([]float32{1.5, -2.0, 0.0})
	if res != "[1.500000,-2.000000,0.000000]" {
		t.Errorf("unexpected output: %s", res)
	}
}
