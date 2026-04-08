package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService_Flow(t *testing.T) {
	ctx := context.Background()

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqlDB.Close()

	dbProvider := db.NewSqliteProvider(sqlDB)

	// Initialize tables directly for test
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	service := NewRAGSyncService(dbProvider)

	// Setup Initial Data
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES
			('1', 'Test Context 1', 'pending', NULL),
			('2', 'Test Context 2', 'pending', NULL),
			('3', 'Test Context 3', 'synced', CURRENT_TIMESTAMP)
	`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	ids := []string{"1", "2"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "Test Context 4", Vector: []float32{0.7, 0.8}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "4" {
		t.Fatalf("expected record ID '4', got %s", pending[0].ID)
	}
}
