package memory_test

import (
	"context"
	"testing"
	"time"
	"database/sql"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
)

func TestVectorRepository(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
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

	repo := memory.NewVectorRepository(provider)

	record1 := &memory.EmbeddingRecord{
		ID:           "test-id-1",
		OrganizationID: "test-org",
		MemoryType:   "TASK_SUMMARY",
		Content:      "test-content-1",
		Embedding:    []float32{1.0, 0.0, 0.0},
		CreatedAt:    time.Now().Add(-24 * time.Hour),
		SourceTaskID: "task-1",
	}
	record2 := &memory.EmbeddingRecord{
		ID:           "test-id-2",
		OrganizationID: "test-org",
		MemoryType:   "TASK_SUMMARY",
		Content:      "test-content-2",
		Embedding:    []float32{0.0, 1.0, 0.0},
		CreatedAt:    time.Now().Add(-48 * time.Hour),
		SourceTaskID: "task-2",
	}

	// Test Upsert
	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("Upsert failed: %v", err)
	}
	if err := repo.Upsert(ctx, record2); err != nil {
		t.Fatalf("Upsert failed: %v", err)
	}

	// Test SemanticSearch (In-Memory SQLite path)
	queryEmb := []float32{1.0, 0.0, 0.0} // Should perfectly match record1
	results, err := repo.SemanticSearch(ctx, "test-org", queryEmb, 2)
	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}
	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}
	if results[0].Record.ID != "test-id-1" {
		t.Errorf("expected top result to be test-id-1, got %s", results[0].Record.ID)
	}
	if results[0].Score < 0.99 {
		t.Errorf("expected score ~1.0, got %f", results[0].Score)
	}

	// Test UpdateRecord
	record1.Content = "updated-content"
	if err := repo.UpdateRecord(ctx, record1); err != nil {
		t.Fatalf("UpdateRecord failed: %v", err)
	}
	// Verify update via semantic search
	results2, err := repo.SemanticSearch(ctx, "test-org", queryEmb, 1)
	if err != nil || len(results2) == 0 {
		t.Fatalf("SemanticSearch failed or empty after update")
	}
	if results2[0].Record.Content != "updated-content" {
		t.Errorf("expected updated content, got %s", results2[0].Record.Content)
	}

	// Test DeleteOldMemories
	// Both records are older than 12 hours. Let's delete ones older than 36 hours.
	if err := repo.DeleteOldMemories(ctx, "TASK_SUMMARY", 36*time.Hour); err != nil {
		t.Fatalf("DeleteOldMemories failed: %v", err)
	}
	// Verify only record1 is left
	results3, err := repo.SemanticSearch(ctx, "test-org", queryEmb, 5)
	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}
	if len(results3) != 1 {
		t.Fatalf("expected 1 record after deletion, got %d", len(results3))
	}
	if results3[0].Record.ID != "test-id-1" {
		t.Errorf("expected remaining record to be test-id-1, got %s", results3[0].Record.ID)
	}
}
