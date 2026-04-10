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
	// Setup in-memory SQLite DB
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Initialize tables
	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test Insert & FetchPendingSyncs
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'memory 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert record 1: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'memory 2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert record 2: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending syncs, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "2" {
		t.Errorf("expected 1 pending sync with ID 2, got %v", pending)
	}

	// Test ProcessIncomingSync (Insert)
	newRecords := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "memory 3 from cloud",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '3'")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to query new record: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected sync_status synced, got %s", status)
	}

	// Test ProcessIncomingSync (Update)
	updateRecords := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "updated memory 2 from cloud",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, updateRecords)
	if err != nil {
		t.Fatalf("failed to process incoming sync for update: %v", err)
	}

	row = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '2'")
	var content string
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected sync_status synced, got %s", status)
	}
	if content != "updated memory 2 from cloud" {
		t.Errorf("expected updated content, got %s", content)
	}
}
