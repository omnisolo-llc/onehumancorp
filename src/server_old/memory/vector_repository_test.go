package memory

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestVectorRepository(t *testing.T) {
	provider := db.NewTestProvider(t)
	repo := NewVectorRepository(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
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

	record := &EmbeddingRecord{
		ID:             "test-1",
		OrganizationID: "org-1",
		AgentID:        "agent-1",
		Content:        "test content",
		Embedding:      []float32{0.1, 0.2, 0.3},
		SourceType:     "TASK_SUMMARY",
		CreatedAt:      time.Now(),
	}

	if err := repo.Upsert(ctx, record); err != nil {
		t.Errorf("Upsert failed: %v", err)
	}

	// Verify upsert via semantic search (fallback for SQLite returns latest)
	results, err := repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2, 0.3}, 1)
	if err != nil {
		t.Errorf("SemanticSearch failed: %v", err)
	}
	if len(results) != 1 {
		t.Errorf("Expected 1 result, got %d", len(results))
	} else if results[0].ID != "test-1" {
		t.Errorf("Expected ID test-1, got %s", results[0].ID)
	}

	// Test PruneStale
	oldTime := time.Now().Add(1 * time.Hour)
	if err := repo.PruneStale(ctx, oldTime); err != nil {
		t.Errorf("PruneStale failed: %v", err)
	}

	// Verify pruned
	results, err = repo.SemanticSearch(ctx, "org-1", []float32{0.1, 0.2, 0.3}, 1)
	if err != nil {
		t.Errorf("SemanticSearch failed: %v", err)
	}
	if len(results) != 0 {
		t.Errorf("Expected 0 results after pruning, got %d", len(results))
	}
}
