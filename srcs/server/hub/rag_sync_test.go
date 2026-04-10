package hub

import (
	"context"
	"testing"

    "os"
    "path/filepath"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestSqlRAGSyncService(t *testing.T) {
	ctx := context.Background()

    // Setup an in-memory SQLite database
    dbPath := filepath.Join(t.TempDir(), "test.db")
    os.Setenv("DATABASE_URL", "sqlite://file:"+dbPath+"?mode=memory&cache=shared")
    t.Setenv("DATABASE_URL", "sqlite://file:"+dbPath+"?mode=memory&cache=shared")
    database, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to create db: %v", err)
    }

    // Setup schema
    schema := `
    CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
        memory_id        TEXT PRIMARY KEY,
        context          TEXT NOT NULL,
        vector_embedding BYTEA,
        source_plugin    TEXT,
        created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        sync_status      VARCHAR(50) DEFAULT 'pending',
        last_sync_at     TIMESTAMP NULL
    );`

    _, err = database.Provider.Exec(ctx, schema)
    if err != nil {
        t.Fatalf("failed to create schema: %v", err)
    }

    svc := NewSqlRAGSyncService(database)

    // Insert some pending records
    database.Provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')`)
    database.Provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'ctx2', 'pending')`)

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "3", Context: "ctx3", SyncStatus: SyncStatusPending}})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

    var count int
    database.Provider.QueryRow(ctx, `SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '3'`).Scan(&count)
    if count != 1 {
        t.Fatalf("expected 1 record with id 3, got %d", count)
    }
}
