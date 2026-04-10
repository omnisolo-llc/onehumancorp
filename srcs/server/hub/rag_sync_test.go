package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
	// Setup in-memory SQLite for testing
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	// Needs schema to test
	_, err = sqlDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	service := NewDefaultRAGSyncService(provider, nil)
	ctx := context.Background()

	// Insert initial data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].Context != "test context 1" {
		t.Errorf("expected context 'test context 1', got '%s'", records[0].Context)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// Verify it was marked synced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	// Verify insertion
	var count int
	err = sqlDB.QueryRow("SELECT COUNT(*) FROM autodream_memories WHERE id = '2'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count records: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record with id 2, got %d", count)
	}
}
