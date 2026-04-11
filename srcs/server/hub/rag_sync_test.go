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

func TestRAGSyncService(t *testing.T) {
	tmpfile, err := os.CreateTemp("", "testdb_*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp db file: %v", err)
	}
	defer os.Remove(tmpfile.Name())

	sqldb, err := sql.Open("sqlite", tmpfile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite database: %v", err)
	}
	defer sqldb.Close()

	provider := db.NewSqliteProvider(sqldb)

	ctx := context.Background()

	// Initialize schema
	schema := `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`
	if _, err := provider.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert test data
	now := time.Now()
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "1", "ctx1", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES (?, ?, ?, ?)", "2", "ctx2", "synced", now)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Errorf("expected pending record ID 1, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after MarkSynced failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "ctx3",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT context, vector_embedding FROM swarm_memory_embeddings WHERE memory_id = ?", "3")
	var contextVal string
	var vectorBytes []byte
	if err := row.Scan(&contextVal, &vectorBytes); err != nil {
		t.Fatalf("failed to verify ProcessIncomingSync: %v", err)
	}
	if contextVal != "ctx3" {
		t.Errorf("expected context ctx3, got %s", contextVal)
	}
}
