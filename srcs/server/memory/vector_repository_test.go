package memory

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestVectorRepository(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	// In test, creating table
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewVectorRepository(provider)

	now := time.Now().Truncate(time.Second)

	record1 := &EmbeddingRecord{
		ID:             "mem-1",
		OrganizationID: "org-1",
		MemoryType:     "task",
		Content:        "Maya's cake is $50",
		Embedding:      []float32{1.0, 0.0, 0.0},
		CreatedAt:      now.Add(-40 * 24 * time.Hour),
	}

	record2 := &EmbeddingRecord{
		ID:             "mem-2",
		OrganizationID: "org-1",
		MemoryType:     "task",
		Content:        "Maya's cake is $55", // Conflict
		Embedding:      []float32{0.9, 0.1, 0.0},
		CreatedAt:      now.Add(-10 * 24 * time.Hour),
	}

	if err := repo.Upsert(ctx, record1); err != nil {
		t.Fatalf("failed to upsert record1: %v", err)
	}
	if err := repo.Upsert(ctx, record2); err != nil {
		t.Fatalf("failed to upsert record2: %v", err)
	}

	// Test SemanticSearch
	results, err := repo.SemanticSearch(ctx, "org-1", []float32{1.0, 0.0, 0.0}, 2)
	if err != nil {
		t.Fatalf("failed to search: %v", err)
	}

	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}

	recent, err := repo.FindRecentMemories(ctx, "org-1", now.Add(-20*24*time.Hour))
	if err != nil {
		t.Fatalf("failed find recent: %v", err)
	}
	if len(recent) != 1 || recent[0].ID != "mem-2" {
		t.Fatalf("expected mem-2")
	}

	err = repo.Delete(ctx, "mem-1")
	if err != nil {
		t.Fatalf("failed to delete: %v", err)
	}

	count, _ := repo.PruneStaleMemories(ctx, time.Now())
	if count != 0 {
		t.Fatalf("expected 0 prune")
	}
}

// Test cosine similarity edge cases
func TestCosineSimilarity(t *testing.T) {
	if cosineSimilarity([]float32{1, 0}, []float32{0, 1}) != 0 {
		t.Fatalf("expected 0")
	}
	if cosineSimilarity([]float32{1, 0}, []float32{1, 0}) != 1 {
		t.Fatalf("expected 1")
	}
	if cosineSimilarity([]float32{0, 0}, []float32{0, 0}) != 0 {
		t.Fatalf("expected 0 for zero vectors")
	}
	if cosineSimilarity([]float32{1, 0}, []float32{1, 0, 0}) != 0 {
		t.Fatalf("expected 0 for different lengths")
	}
}

type MockPgProvider struct {
	db.Provider
}

func (m *MockPgProvider) IsSQLite() bool {
	return false
}

func (m *MockPgProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, nil
}

func (m *MockPgProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &MockPgRows{}, nil
}

type MockPgRows struct {
	count int
}
func (m *MockPgRows) Next() bool {
	if m.count == 0 {
		m.count++
		return true
	}
	return false
}
func (m *MockPgRows) Scan(dest ...any) error {
	v0 := dest[0].(*string)
	*v0 = "mem-pg"
	v1 := dest[1].(*string)
	*v1 = "org-1"
	v3 := dest[3].(*string)
	*v3 = "task"
	v4 := dest[4].(*string)
	*v4 = "test content pg"
	v5 := dest[5].(*string)
	*v5 = "[1.0, 0.0]"
	return nil
}
func (m *MockPgRows) Close() {}
func (m *MockPgRows) Columns() ([]string, error) { return nil, nil }
func (m *MockPgRows) Err() error { return nil }

func TestPgVectorRepository(t *testing.T) {
	ctx := context.Background()
	repo := NewVectorRepository(&MockPgProvider{})

	record := &EmbeddingRecord{
		ID:             "mem-pg",
		OrganizationID: "org-1",
		MemoryType:     "task",
		Content:        "test content pg",
		Embedding:      []float32{1.0, 0.0},
		CreatedAt:      time.Now(),
	}
	err := repo.Upsert(ctx, record)
	if err != nil {
		t.Fatalf("failed pg upsert: %v", err)
	}

	results, err := repo.SemanticSearch(ctx, "org-1", []float32{1.0, 0.0}, 1)
	if err != nil {
		t.Fatalf("failed pg semantic search: %v", err)
	}

	if len(results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(results))
	}

	if results[0].ID != "mem-pg" {
		t.Fatalf("expected mem-pg, got %s", results[0].ID)
	}

	_, err = repo.FindRecentMemories(ctx, "org-1", time.Now())
	if err != nil {
		t.Fatalf("failed pg find recent: %v", err)
	}
}
