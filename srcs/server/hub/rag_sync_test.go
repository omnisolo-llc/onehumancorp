package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	ctx := context.Background()

	// Setup schema
	_, err = provider.Exec(ctx, `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_timestamp DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'test context', '[0.1, 0.2]', 'pending');
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got %s", records[0].ID)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected status pending, got %v", records[0].SyncStatus)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify MarkSynced
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}

	// Test ProcessIncomingSync
	incomingTime := time.Now().Truncate(time.Second)
	incomingRecords := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			Vector:     []float32{0.3, 0.4},
			SyncStatus: SyncStatusPending,
			LastSyncAt: incomingTime,
		},
	}
	err = svc.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = '2' AND sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query incoming count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 incoming record inserted, got %d", count)
	}
}
