package hub

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestSQLRAGSyncService(t *testing.T) {
	ctx := context.Background()

    // db.New uses DATABASE_URL environment variable
    os.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    defer os.Unsetenv("DATABASE_URL")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	service := NewSQLRAGSyncService(database)

	// Test ProcessIncomingSync
	now := time.Now().Truncate(time.Second)
	records := []RAGSyncRecord{
		{
			ID:         "11111111-1111-1111-1111-111111111111",
			Context:    "test context 1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
		{
			ID:         "22222222-2222-2222-2222-222222222222",
			Context:    "test context 2",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
	}

	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	ids := []string{"11111111-1111-1111-1111-111111111111"}
	if err := service.MarkSynced(ctx, ids); err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs after mark: %v", err)
	}

	if len(pendingAfter) != 1 {
		t.Fatalf("expected 1 pending record after mark, got %d", len(pendingAfter))
	}

	if pendingAfter[0].ID != "22222222-2222-2222-2222-222222222222" {
		t.Errorf("expected ID '22222222-2222-2222-2222-222222222222', got '%s'", pendingAfter[0].ID)
	}
}
