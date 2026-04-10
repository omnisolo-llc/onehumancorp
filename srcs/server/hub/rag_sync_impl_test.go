package hub

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
    "database/sql"
)

func TestDefaultRAGSyncService(t *testing.T) {
	InitRAGSyncMetrics(nil)

    t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
    ctx := context.Background()
    provider, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to create db provider: %v", err)
    }
    defer provider.Close()

    // Create necessary table since it's an in-memory DB and might not run migrations by default
    _, err = provider.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            sync_status      TEXT DEFAULT 'pending',
            last_sync_at     TIMESTAMPTZ NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	svc := NewDefaultRAGSyncService(provider)

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "incoming context",
			SyncStatus: SyncStatusPending,
		},
	}
	err = svc.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

    // Insert a pending record manually to test FetchPendingSyncs
    _, err = provider.Exec(ctx, `
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
        VALUES ('2', 'pending context', '[0.3, 0.4]', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert pending record: %v", err)
    }

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(records))
	}
    if records[0].ID != "2" {
        t.Fatalf("expected record ID 2, got %s", records[0].ID)
    }

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

    // Verify it was marked as synced
    var status string
    var lastSyncAt sql.NullString
    row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = '2'")
    err = row.Scan(&status, &lastSyncAt)
    if err != nil {
        t.Fatalf("failed to verify status: %v", err)
    }
    if status != string(SyncStatusSynced) {
        t.Fatalf("expected status %s, got %s", SyncStatusSynced, status)
    }
    if !lastSyncAt.Valid {
        t.Fatalf("expected last_sync_at to be set, got null")
    }

    // Test ProcessIncomingSync with existing ID (ON CONFLICT DO UPDATE)
    updateRecords := []RAGSyncRecord{
        {
            ID: "2",
            Context: "updated context",
        },
    }
    err = svc.ProcessIncomingSync(ctx, updateRecords)
    if err != nil {
        t.Fatalf("ProcessIncomingSync update failed: %v", err)
    }

    var newContext string
    row = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '2'")
    err = row.Scan(&newContext)
    if err != nil {
        t.Fatalf("failed to verify updated context: %v", err)
    }
    if newContext != "updated context" {
        t.Fatalf("expected context 'updated context', got '%s'", newContext)
    }
}
