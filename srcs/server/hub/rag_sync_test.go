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

func TestDefaultRAGSyncService(t *testing.T) {
	// Setup temp sqlite DB
	tmpFile, err := os.CreateTemp("", "testdb-*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	// Create table
	_, err = sqlDB.Exec(`
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT,
        embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	service := NewDefaultRAGSyncService(provider)

	// Insert test data
	_, err = sqlDB.Exec("INSERT INTO autodream_memories (id, content, embedding) VALUES ('test-1', 'test content', '[0.1]')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	ctx := context.Background()

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
		t.Fatalf("expected 0 pending records, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-2",
			Context:    "updated content",
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}
