package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRagSyncServiceImpl(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	// Initialize the schema
	_, err = sqliteDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	service := NewRAGSyncService(dbWrapper)

	ctx := context.Background()

	// 1. Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{
			ID:      "mem1",
			Context: "some knowledge",
			Vector:  []float32{0.1, 0.2, 0.3},
		},
	}
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. Fetch it back to verify it was inserted.
	// Since ProcessIncomingSync marks it as "synced", FetchPendingSyncs shouldn't find it.
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending syncs, got %d", len(pending))
	}

	// Insert a pending record manually
	_, err = sqliteDB.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem2', 'local thought', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// 3. Test FetchPendingSyncs
	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem2" {
		t.Fatalf("expected 1 pending sync mem2, got %v", pending)
	}

	// 4. Test MarkSynced
	err = service.MarkSynced(ctx, []string{"mem2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify no pending left
	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending syncs after MarkSynced, got %d", len(pending))
	}
}
