package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
	ctx := context.Background()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)

	// Create table
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status TEXT,
			last_sync_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES
			('id1', 'ctx1', 'pending'),
			('id2', 'ctx2', 'pending'),
			('id3', 'ctx3', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	svc := NewDefaultRAGSyncService(provider)

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "id4", Context: "ctx4", Vector: []float32{1.0, 2.0}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"id1", "id2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify all pending are synced
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pending))
	}
}
