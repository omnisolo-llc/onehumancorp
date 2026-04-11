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

func TestSQLRAGSyncService(t *testing.T) {
	ctx := context.Background()

	tmpFile, err := os.CreateTemp("", "test_db_*.sqlite")
	if err != nil {
		t.Fatalf("Failed to create temp db: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	svc := NewSQLRAGSyncService(provider)

	// Insert initial data directly
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "mem1", "context 1", "pending")
	if err != nil {
		t.Fatalf("Failed to insert mem1: %v", err)
	}

	// 1. FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending, got %d", len(pending))
	}
	if pending[0].ID != "mem1" {
		t.Errorf("Expected id mem1, got %s", pending[0].ID)
	}

	// 2. MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Check it was marked
	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Errorf("Expected 0 pending, got %d", len(pendingAfter))
	}

	// 3. ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "mem2",
			Context:    "context 2 cloud",
			Vector:     nil,
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT count(*) FROM swarm_memory_embeddings")
	if err != nil {
		t.Fatalf("Query count failed: %v", err)
	}
	defer rows.Close()
	if rows.Next() {
		var count int
		if err := rows.Scan(&count); err != nil {
			t.Fatalf("Scan count failed: %v", err)
		}
		if count != 2 {
			t.Errorf("Expected 2 total rows, got %d", count)
		}
	}
}
