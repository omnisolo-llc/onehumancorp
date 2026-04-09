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
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqldb.Close()

	dbProvider := db.NewSqliteProvider(sqldb)

	// Setup schema using exactly the columns discovered in 005_sip.sql and the new ones
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(dbProvider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "ctx1", SyncStatus: SyncStatusPending},
	}
	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem1" {
		t.Fatalf("expected 1 pending record with ID mem1, got %v", pending)
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem1"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify no pending syncs
	pending2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending2) != 0 {
		t.Fatalf("expected 0 pending records, got %v", len(pending2))
	}
}
