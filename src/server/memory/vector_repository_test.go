package memory_test

import (
	"context"
	"testing"
	"time"
	"database/sql"
	"strings"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
)

// mockProvider intercepts queries to replace vec_distance_cosine with a constant 0.0 for testing,
// simulating a perfect semantic match to pass the score > 0.90 threshold.
type mockProvider struct {
	db.Provider
}

func (m *mockProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
	if strings.Contains(query, "vec_distance_cosine") {
		// Replace vec_distance_cosine(embedding, $2) with 0.0 to simulate perfect match
		query = strings.ReplaceAll(query, "vec_distance_cosine(embedding, $2)", "0.0")
	}
	return m.Provider.Query(ctx, query, args...)
}

func TestVectorRepository(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	sqliteProvider := db.NewSqliteProvider(dbConn)
	provider := &mockProvider{Provider: sqliteProvider}
	ctx := context.Background()

	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories_master (
		id VARCHAR PRIMARY KEY,
		tenant_id VARCHAR NOT NULL,
		memory_type TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		source_task_id VARCHAR
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := memory.NewVectorRepository(provider)

	record1 := &memory.EmbeddingRecord{
		ID:           "test-id-1",
		TenantID:     "test-org",
		MemoryType:   "TASK_SUMMARY",
		Content:      "test-content-1",
		Embedding:    []float32{1.0, 0.0, 0.0},
		CreatedAt:    time.Now().Add(-24 * time.Hour),
		SourceTaskID: "task-1",
	}
	record2 := &memory.EmbeddingRecord{
		ID:           "test-id-2",
		TenantID:     "test-org",
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

	// Test SemanticSearch (In-Memory SQLite path using mocked provider)
	queryEmb := []float32{1.0, 0.0, 0.0}
	results, err := repo.SemanticSearch(ctx, "test-org", queryEmb, 2)
	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}
	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}
	// The results order is arbitrary since all scores are simulated as 1.0
	// but we expect both to be returned and score = 1.0
	for _, r := range results {
		if r.Score != 1.0 {
			t.Errorf("expected score 1.0, got %f", r.Score)
		}
	}

	// Test UpdateRecord
	record1.Content = "updated-content"
	if err := repo.UpdateRecord(ctx, record1); err != nil {
		t.Fatalf("UpdateRecord failed: %v", err)
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
	if results3[0].Record.Content != "updated-content" {
		t.Errorf("expected updated-content, got %s", results3[0].Record.Content)
	}
}
