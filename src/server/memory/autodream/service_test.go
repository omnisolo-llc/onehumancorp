package autodream

import (
	"context"
	"testing"
	"database/sql"
	"time"
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

func TestAutoDreamConsolidation(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

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
	// Record is created now, so 30 days retention won't delete it
	err = service.PruneStaleContext(ctx, 30 * 24 * time.Hour)
	if err != nil {
		t.Fatalf("PruneStaleContext failed: %v", err)
	}
	resultsAfterPrune, _ := repo.SemanticSearch(ctx, claims.OrganizationID, []float32{0.1, 0.2, 0.3}, 10)
	if len(resultsAfterPrune) != 1 {
		t.Errorf("expected 1 result after prune, got %d", len(resultsAfterPrune))
	}
}
