package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}

	// Create required schema
	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc, err := NewSqliteRAGSyncService(provider)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	ctx := context.Background()

	// 1. Insert initial pending records directly via provider
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "1" {
		t.Fatalf("Expected 1 pending record with ID 1, got %v", pending)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %v", pendingAfter)
	}

	// Test ProcessIncomingSync (upsert logic)
	now := time.Now().Truncate(time.Second) // Truncate to second for SQLite comparison
	records := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "test context 2",
			Vector:     []float32{1.1, 2.2},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	row := provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE id = '2' AND sync_status = 'synced'")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("Failed to verify process incoming sync: %v", err)
	}
	if count != 1 {
		t.Fatalf("Expected 1 record with ID 2 to be synced, got %d", count)
	}
}
