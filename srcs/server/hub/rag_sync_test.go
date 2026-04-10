package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	defer sqliteDB.Close()

	// Initialize schema for test
	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(sqliteDB)
	ctx := context.Background()

	// Insert test data
	_, err = sqliteDB.Exec(`
		INSERT INTO autodream_memories (id, content, sync_status) VALUES
		('1', 'test context 1', 'pending'),
		('2', 'test context 2', 'pending'),
		('3', 'test context 3', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}

	// Test MarkSynced
	idsToMark := []string{"1", "2"}
	err = svc.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify MarkSynced worked
	recordsAfterMark, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(recordsAfterMark) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(recordsAfterMark))
	}

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{ID: "4", Context: "test context 4", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}, // new record (insert)
		{ID: "1", Context: "test context 1 updated", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}, // existing record (update)
	}
	err = svc.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	err = sqliteDB.QueryRow(`SELECT count(*) FROM autodream_memories WHERE id IN ('4', '1') AND sync_status = 'synced'`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Errorf("expected 2 records updated/inserted, got %d", count)
	}
}
