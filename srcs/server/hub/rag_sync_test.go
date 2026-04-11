package hub

import (
	"context"
	"testing"
	"database/sql"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncServiceFlow(t *testing.T) {
	ctx := context.Background()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)
	defer provider.Close()

	// Create table manually for test
	_, err = provider.Exec(ctx, `CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Insert dummy data directly
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ($1, $2, $3)", "1", "ctx1", SyncStatusPending)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ($1, $2, $3)", "2", "ctx2", SyncStatusPending)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	ids := []string{"1", "2"}
	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pending))
	}

	records := []RAGSyncRecord{
		{ID: "3", Context: "ctx3"},
	}
	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
