package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)
	defer provider.Close()

	// 1. Manually setup table
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending'), ('2', 'ctx2', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Fetch
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("Expected 2 records, got %d", len(records))
	}

	// Mark Synced
	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("Failed to mark synced: %v", err)
	}

	// Verify marked synced
	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending after mark synced: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 records, got %d", len(records))
	}

	// Process incoming sync
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "ctx3", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("Failed to process incoming sync: %v", err)
	}
}
