package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"database/sql"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp TIMESTAMP NULL
		);
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'test 1', x'0102', 'pending');
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('2', 'test 2', x'0304', 'synced');
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('3', 'test 3', x'0506', 'pending');
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	service := NewRAGSyncService(provider)

	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		pending, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(pending) != 2 {
			t.Fatalf("expected 2 pending records, got %d", len(pending))
		}
		if pending[0].Vector == nil || len(pending[0].Vector) == 0 {
			t.Fatalf("expected vector to be fetched")
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := service.MarkSynced(ctx, []string{"1"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		var status string
		err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status)
		if err != nil {
			t.Fatalf("failed to query status: %v", err)
		}

		if status != string(SyncStatusSynced) {
			t.Fatalf("expected status %s, got %s", SyncStatusSynced, status)
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		newRecord := RAGSyncRecord{ID: "4", Context: "new record", Vector: []byte{7, 8}, SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}
		err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{newRecord})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		var context string
		var vector []byte
		err = sqlDB.QueryRow("SELECT context, vector_embedding FROM swarm_memory_embeddings WHERE memory_id = '4'").Scan(&context, &vector)
		if err != nil {
			t.Fatalf("failed to query new record: %v", err)
		}
		if context != "new record" {
			t.Fatalf("expected record context 'new record', got %s", context)
		}
		if len(vector) != 2 {
			t.Fatalf("expected vector of length 2")
		}
	})
}
