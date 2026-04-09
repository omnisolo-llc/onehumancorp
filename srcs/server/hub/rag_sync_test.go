package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	_ "modernc.org/sqlite"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize SQLite provider for testing
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	provider := db.NewSqliteProvider(sqldb)

	defer provider.Close()

	// Create necessary schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test data
	vector := []float32{0.1, 0.2, 0.3}
	vectorBytes, _ := json.Marshal(vector)

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('mem1', 'context 1', ?, 'pending'),
		       ('mem2', 'context 2', ?, 'synced')
	`, vectorBytes, vectorBytes)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		records, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("FetchPendingSyncs failed: %v", err)
		}

		if len(records) != 1 {
			t.Fatalf("expected 1 record, got %d", len(records))
		}

		if records[0].ID != "mem1" {
			t.Errorf("expected mem1, got %s", records[0].ID)
		}
		if records[0].SyncStatus != SyncStatusPending {
			t.Errorf("expected pending, got %s", records[0].SyncStatus)
		}
		if len(records[0].Vector) != 3 {
			t.Errorf("expected vector of length 3, got %v", records[0].Vector)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := service.MarkSynced(ctx, []string{"mem1"})
		if err != nil {
			t.Fatalf("MarkSynced failed: %v", err)
		}

		// Verify
		var status string
		err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem1'").Scan(&status)
		if err != nil {
			t.Fatalf("failed to query status: %v", err)
		}
		if status != string(SyncStatusSynced) {
			t.Errorf("expected synced, got %s", status)
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		records := []RAGSyncRecord{
			{
				ID:         "mem3",
				Context:    "context 3",
				Vector:     []float32{0.4, 0.5, 0.6},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		}

		err := service.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("ProcessIncomingSync failed: %v", err)
		}

		// Verify
		var contextStr string
		err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem3'").Scan(&contextStr)
		if err != nil {
			t.Fatalf("failed to query new record: %v", err)
		}
		if contextStr != "context 3" {
			t.Errorf("expected 'context 3', got %s", contextStr)
		}
	})
}
