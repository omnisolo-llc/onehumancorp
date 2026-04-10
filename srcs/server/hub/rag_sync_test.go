package hub_test

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func TestRAGSyncService(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Initialize tables needed
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TEXT DEFAULT CURRENT_TIMESTAMP,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     TEXT NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	svc := hub.NewRAGSyncService(provider)

	t.Run("ProcessIncomingSync and FetchPendingSyncs", func(t *testing.T) {
		records := []hub.RAGSyncRecord{
			{
				ID:      "mem-1",
				Context: "Local knowledge",
				Vector:  []float32{0.1, 0.2, 0.3},
			},
		}

		// Initially insert via normal local process (simulated)
		_, err := provider.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
			VALUES ('mem-2', 'Pending knowledge', 'pending')
		`)
		if err != nil {
			t.Fatalf("Failed to insert pending rec: %v", err)
		}

		// Process incoming sync (synced automatically)
		err = svc.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("ProcessIncomingSync failed: %v", err)
		}

		// Fetch pending syncs
		pending, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("FetchPendingSyncs failed: %v", err)
		}

		if len(pending) != 1 {
			t.Fatalf("Expected 1 pending record, got %d", len(pending))
		}
		if pending[0].ID != "mem-2" {
			t.Errorf("Expected pending record mem-2, got %s", pending[0].ID)
		}

		// Mark synced
		err = svc.MarkSynced(ctx, []string{"mem-2"})
		if err != nil {
			t.Fatalf("MarkSynced failed: %v", err)
		}

		// Fetch again, should be empty
		pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("FetchPendingSyncs after mark failed: %v", err)
		}
		if len(pendingAfter) != 0 {
			t.Errorf("Expected 0 pending records, got %d", len(pendingAfter))
		}
	})
}
