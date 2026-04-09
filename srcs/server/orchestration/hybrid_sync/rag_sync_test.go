package hybrid_sync

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	service := NewRAGSyncService(dbWrapper)
	ctx := context.Background()

	// Insert a pending record
	_, err = sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test_context', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %v", len(records))
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Fatalf("expected pending status, got %v", records[0].SyncStatus)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	err = sqlDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected synced status, got %v", status)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "2", Context: "new_context", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusSynced},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var ctxContent string
	err = sqlDB.QueryRow("SELECT content FROM autodream_memories WHERE id = '2'").Scan(&ctxContent)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if ctxContent != "new_context" {
		t.Fatalf("expected new_context, got %v", ctxContent)
	}
}
