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
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Create table manually since NewTestProvider uses in-memory db
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
        source_plugin TEXT,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMPTZ NULL
	);`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewDBRAGSyncService(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "Test context 1", Vector: []byte{1, 2, 3}},
	}
	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert pending
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('mem2', 'Test context 2', 'blob', 'pending')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem2" {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after MarkSynced failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
