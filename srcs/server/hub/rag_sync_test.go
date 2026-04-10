package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	// Setup in-memory sqlite
	os.Setenv("DATABASE_URL", "sqlite://test.db")
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	defer provider.Close()

	ctx := context.Background()

	// Need to manually create the table for the test since we're using in-memory and migrations aren't automatically applied here.
	// As per memory note: "you must explicitly execute CREATE TABLE setup statements within the test to ensure required tables exist."
	createTableSQL := `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMPTZ NULL
	);
	`
	_, err = provider.Exec(ctx, createTableSQL)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service, err := NewRAGSyncService(provider)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	// 1. ProcessIncomingSync (insert)
	now := time.Now()
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{
			ID:         "mem-1",
			Context:    "test context 1",
			SyncStatus: SyncStatusPending,
		},
		{
			ID:         "mem-2",
			Context:    "test context 2",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	})
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	// 2. FetchPendingSyncs
	pendingRecords, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(pendingRecords) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pendingRecords))
	}
	if pendingRecords[0].ID != "mem-1" {
		t.Errorf("expected pending record ID mem-1, got %s", pendingRecords[0].ID)
	}
	if pendingRecords[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected status pending, got %s", pendingRecords[0].SyncStatus)
	}

	// 3. MarkSynced
	err = service.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	// 4. Verify no pending syncs
	pendingRecords2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs after mark: %v", err)
	}
	if len(pendingRecords2) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingRecords2))
	}
}
