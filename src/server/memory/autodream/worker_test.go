package autodream

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/lib/resilience/lock"
	"github.com/onehumancorp/mono/src/server/memory"
)

func TestBackgroundWorker_Start(t *testing.T) {
	provider := setupTestDB(t)
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)
	lockProv := lock.NewDatabaseLockProvider(provider)

	ctx := context.Background()

	// Insert mock data to create an organization ID
	err := repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "m1",
		OrganizationID: "test-tenant-123",
		Content:        "A",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
	})
	if err != nil {
		t.Fatalf("failed to upsert mock data: %v", err)
	}

	// Create another conflicting mock data
	err = repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "m2",
		OrganizationID: "test-tenant-123",
		Content:        "B",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
	})
	if err != nil {
		t.Fatalf("failed to upsert mock data: %v", err)
	}

	worker := NewBackgroundWorker(service, repo, 100*time.Millisecond, 24*time.Hour, lockProv)

	// Instead of a goroutine that races against context cancellation,
	// test the logic directly using runCycle for determinism.
	// We'll set the context claims for auth checks during ResolveConflicts
	ctxRun := auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "test-tenant-123"})

	worker.runCycle(ctxRun)

	// Verify that conflicts were resolved and stale context was pruned.
	ctxVerify := context.Background()
	records, err := repo.SemanticSearch(ctxVerify, "test-tenant-123", []float32{0.1}, 10)
	if err != nil {
		t.Fatalf("failed to search records: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record (the merged one) after prune, got %d", len(records))
	} else if records[0].ID != "m1-merged" {
		t.Errorf("expected merged record ID 'm1-merged', got %s", records[0].ID)
	}
}
