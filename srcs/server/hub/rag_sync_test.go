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

func TestRAGSyncServiceImpl_SQLite(t *testing.T) {
	ctx := context.Background()

	tmpFile, err := os.CreateTemp("", "test_db_*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	schema := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     TIMESTAMP NULL
	);
	`
	if _, err := sqlDB.Exec(schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	dbWrapper := db.NewSqliteProvider(sqlDB)
	svc := NewRAGSyncService(dbWrapper)

	// Insert test data
	now := time.Now().UTC()
	insertQuery := `
	INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
	VALUES
		('mem1', 'ctx1', X'0001', 'pending', NULL),
		('mem2', 'ctx2', X'0002', 'synced', ?)
	`
	if _, err := sqlDB.Exec(insertQuery, now); err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem1" {
		t.Errorf("expected mem1, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify mem1 is marked as synced
	var status string
	err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}

	// Test ProcessIncomingSync (should fail for SQLite)
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "mem3"}})
	if err == nil {
		t.Errorf("expected ProcessIncomingSync to fail for SQLite")
	}
}
