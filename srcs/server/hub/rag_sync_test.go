package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqlDB.Close()

	dbWrapper := db.NewSqliteProvider(sqlDB)

	_, err = dbWrapper.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	_, err = dbWrapper.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-1', 'some context', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(dbWrapper)

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-1" {
		t.Errorf("expected ID 'test-1', got '%s'", records[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-2",
			Context:    "remote context",
			SyncStatus: SyncStatusSynced,
			Vector:     []float32{0.1, 0.2, 0.3},
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var content string
	var syncStatus string
	err = dbWrapper.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'test-2'").Scan(&content, &syncStatus)
	if err != nil {
		t.Fatalf("expected no error verifying ProcessIncomingSync, got %v", err)
	}
	if content != "remote context" {
		t.Errorf("expected content 'remote context', got '%s'", content)
	}
	if syncStatus != "synced" {
		t.Errorf("expected sync_status 'synced', got '%s'", syncStatus)
	}
}
