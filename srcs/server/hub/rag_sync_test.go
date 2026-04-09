package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

func NewTestDB(t *testing.T) *db.DB {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Create necessary table manually to bypass migration issues in isolation
	_, err = sqliteDB.ExecContext(context.Background(), `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMPTZ NULL
	);
	`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize test database directly to avoid global migration failures in isolation
	dbProvider := NewTestDB(t)

	svc := hub.NewRAGSyncService(dbProvider)

	// Insert dummy record
	dummyID := "e99f0e1c-b8b5-4b52-b883-9b882eb7d8c5"
	_, err := dbProvider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ($1, $2, $3)", dummyID, "test memory", string(hub.SyncStatusPending))
	if err != nil {
		t.Fatalf("Failed to insert dummy record: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != dummyID {
		t.Errorf("expected ID %s, got %s", dummyID, records[0].ID)
	}
	if records[0].SyncStatus != hub.SyncStatusPending {
		t.Errorf("expected status %s, got %s", hub.SyncStatusPending, records[0].SyncStatus)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{dummyID})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	// Verify it was marked synced
	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incomingRecord := hub.RAGSyncRecord{
		ID:         "f10f0e1c-b8b5-4b52-b883-9b882eb7d8c6",
		Context:    "incoming context",
		Vector:     []float32{0.1, 0.2, 0.3},
		SyncStatus: hub.SyncStatusSynced,
		LastSyncAt: time.Now(),
	}

	err = svc.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{incomingRecord})
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}
}
