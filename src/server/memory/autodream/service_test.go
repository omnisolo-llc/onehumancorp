package autodream

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"

	import_sqlite "modernc.org/sqlite"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	_ = import_sqlite.RegisterDeterministicScalarFunction("vec_distance_cosine", 2, func(ctx *import_sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		return 0.01, nil
	})

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()
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
	return provider
}

func TestAutoDreamConsolidation(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	err := service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestResolveConflicts(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert mock data
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m2", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})

	err := service.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestPruneStaleContext(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Insert mock data with old date
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1}, CreatedAt: time.Now().Add(-48 * time.Hour),
	})

	err := service.PruneStaleContext(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Verify it was deleted
	records, _ := repo.SemanticSearch(ctx, "test-tenant-123", []float32{0.1}, 10)
	if len(records) != 0 {
		t.Errorf("expected 0 records, got %d", len(records))
	}
}
