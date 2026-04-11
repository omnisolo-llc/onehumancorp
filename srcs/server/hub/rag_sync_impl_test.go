package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	localProvider := db.NewTestProvider(t)
	cloudProvider := db.NewTestProvider(t)

	ctx := context.Background()

	// Initialize the tables
	_, err := localProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = cloudProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}


	// Insert some pending syncs into local DB
	_, err = localProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('id1', 'context 1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert record: %v", err)
	}
	_, err = localProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('id2', 'context 2', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert record: %v", err)
	}

	svc := NewRAGSyncService(localProvider, cloudProvider)

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		records, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("FetchPendingSyncs failed: %v", err)
		}
		if len(records) != 2 {
			t.Errorf("Expected 2 records, got %d", len(records))
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		records := []RAGSyncRecord{
			{ID: "id1", Context: "context 1"},
			{ID: "id2", Context: "context 2"},
		}

		err := svc.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("ProcessIncomingSync failed: %v", err)
		}

		var count int
		err = cloudProvider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
		if err != nil {
			t.Fatalf("Failed to check cloud DB count: %v", err)
		}
		if count != 2 {
			t.Errorf("Expected 2 records in cloud DB, got %d", count)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := svc.MarkSynced(ctx, []string{"id1", "id2"})
		if err != nil {
			t.Fatalf("MarkSynced failed: %v", err)
		}

		var count int
		err = localProvider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE sync_status = 'pending'").Scan(&count)
		if err != nil {
			t.Fatalf("Failed to check local DB count: %v", err)
		}
		if count != 0 {
			t.Errorf("Expected 0 pending records in local DB, got %d", count)
		}
	})

	t.Run("ProcessIncomingSync_Update", func(t *testing.T) {
		records := []RAGSyncRecord{
			{ID: "id1", Context: "updated context 1"},
		}

		err := svc.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("ProcessIncomingSync failed: %v", err)
		}

		var context string
		err = cloudProvider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'id1'").Scan(&context)
		if err != nil {
			t.Fatalf("Failed to check cloud DB context: %v", err)
		}
		if context != "updated context 1" {
			t.Errorf("Expected updated context, got %s", context)
		}
	})
}
