package autodream

import (
	"context"
	"testing"
	"database/sql"
	"time"
	"strings"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/memory"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock summary", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

// mockProvider intercepts queries to replace vec_distance_cosine with a constant 0.0 for testing,
// since registering UDFs in modernc.org/sqlite is complex.
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

func TestAutoDreamConsolidation(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	sqliteProvider := db.NewSqliteProvider(dbConn)
	provider := &mockProvider{Provider: sqliteProvider}

	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

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
	llm := &mockLLM{}
	service := NewService(repo, llm)

	// Consolidate first time
	err = service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Consolidate second time with same mock embeddings -> should trigger conflict resolution and update
	err = service.Consolidate(ctx, "task-124", []string{"log 3", "log 4"})
	if err != nil {
		t.Fatalf("expected no error during conflict resolution, got %v", err)
	}

	// Verify only 1 record exists instead of 2, due to conflict resolution merging
	results, err := repo.SemanticSearch(ctx, claims.OrganizationID, []float32{0.1, 0.2, 0.3}, 10)
	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}
	if len(results) != 1 {
		t.Errorf("expected 1 result due to conflict resolution merge, got %d", len(results))
	}
	if results[0].Record.SourceTaskID != "task-124" {
		t.Errorf("expected updated task ID to be task-124, got %s", results[0].Record.SourceTaskID)
	}

	// Test PruneStaleContext
	err = service.PruneStaleContext(ctx, 30 * 24 * time.Hour)
	if err != nil {
		t.Fatalf("PruneStaleContext failed: %v", err)
	}
	resultsAfterPrune, _ := repo.SemanticSearch(ctx, claims.OrganizationID, []float32{0.1, 0.2, 0.3}, 10)
	if len(resultsAfterPrune) != 1 {
		t.Errorf("expected 1 result after prune, got %d", len(resultsAfterPrune))
	}
}
