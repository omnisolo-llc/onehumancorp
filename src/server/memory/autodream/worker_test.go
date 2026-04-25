package autodream

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/lib/resilience/lock"
	"github.com/onehumancorp/mono/src/server/memory"
)

func TestBackgroundWorker_Start(t *testing.T) {
	provider := db.NewTestProvider(t)
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := NewService(repo, llm)
	lockProv := lock.NewDatabaseLockProvider(provider)

	ctx := context.Background()

	// In test, creating table
	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		source_type TEXT,
		content TEXT NOT NULL,
		embedding BLOB,
		created_at DATETIME,
		updated_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create consolidated_memory table: %v", err)
	}

	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS distributed_locks (
		key TEXT PRIMARY KEY,
		token TEXT NOT NULL,
		expires_at DATETIME NOT NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create distributed_locks table: %v", err)
	}

	// Also create the users table to avoid "no such table: users" error when GetOrganizationIDs is called
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS users (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create users table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO users (id, organization_id) VALUES ('u1', 'test-tenant-123')`)
	if err != nil {
		t.Fatalf("failed to insert user: %v", err)
	}

	// Insert mock data to create an organization ID
	err = repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID:             "m1",
		OrganizationID: "test-tenant-123",
		Content:        "A",
		SourceType:     "TASK_SUMMARY",
		Embedding:      []float32{0.1},
		CreatedAt:      time.Now().Add(-48 * time.Hour),
	})
	if err != nil {
		t.Fatalf("failed to upsert mock data: %v", err)
	}

	worker := NewBackgroundWorker(service, repo, 100*time.Millisecond, 24*time.Hour, lockProv)

	// We'll just run one cycle manually to test the logic
	worker.runCycle(ctx)

	// Pruning should have removed m1 because it's older than 24 hours
	tenantCtx := auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "test-tenant-123"})
	results, err := repo.SemanticSearch(tenantCtx, "test-tenant-123", []float32{0.1}, 10)
	if err != nil {
		t.Fatalf("search failed: %v", err)
	}
	if len(results) != 0 {
		t.Errorf("expected 0 results after pruning, got %d", len(results))
	}
}
