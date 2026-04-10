package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// newTestProvider creates a local SQLite test provider. We do this because db.NewTestProvider is only in the _test.go file of the db package, meaning it isn't exported to other packages.
func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqlDB.Close()
	})

	return db.NewSqliteProvider(sqlDB)
}

func TestRAGSyncService(t *testing.T) {
	provider := newTestProvider(t)
	ctx := context.Background()

	// Setup schema for test
	_, err := provider.Exec(ctx, `CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT,
		embedding TEXT,
		sync_status TEXT,
		last_sync_at TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	service := NewDBRAGSyncService(provider)

	// Test ProcessIncomingSync
	now := time.Now().UTC()
	records := []RAGSyncRecord{
		{ID: "1", Context: "data1", Vector: []float32{1.0, 2.0}, SyncStatus: SyncStatusPending, LastSyncAt: now},
		{ID: "2", Context: "data2", Vector: []float32{3.0, 4.0}, SyncStatus: SyncStatusPending, LastSyncAt: now},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending, got %d", len(pending))
	}
	if len(pending[0].Vector) != 2 || pending[0].Vector[0] != 1.0 {
		t.Fatalf("Expected vector to be extracted properly, got %v", pending[0].Vector)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 1 || pendingAfter[0].ID != "2" {
		t.Fatalf("Expected 1 pending with ID 2, got %v", pendingAfter)
	}
}
