package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // Use the exact bazel dependency name org_modernc_sqlite translates to modernc.org/sqlite, wait, let's look at db_test.
)

func TestRAGSyncService(t *testing.T) {
	// Setup
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Create table schema
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)
	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		// Insert test data
		vecData, _ := json.Marshal([]float32{1.1, 2.2})
		_, err := provider.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
			VALUES (?, ?, ?, 'pending')
		`, "test-id-1", "test context", vecData)
		if err != nil {
			t.Fatalf("failed to insert test data: %v", err)
		}

		records, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		if len(records) != 1 {
			t.Fatalf("expected 1 record, got %d", len(records))
		}

		if records[0].ID != "test-id-1" {
			t.Errorf("expected id test-id-1, got %s", records[0].ID)
		}
		if len(records[0].Vector) != 2 || records[0].Vector[0] != 1.1 {
			t.Errorf("expected vector [1.1, 2.2], got %v", records[0].Vector)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := service.MarkSynced(ctx, []string{"test-id-1"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		// Verify update
		row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_timestamp FROM swarm_memory_embeddings WHERE memory_id = 'test-id-1'")
		var status string
		var lastSync *time.Time
		if err := row.Scan(&status, &lastSync); err != nil {
			t.Fatalf("failed to scan row: %v", err)
		}

		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
		if lastSync == nil {
			t.Error("expected last_sync_timestamp to be set, got nil")
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		record := RAGSyncRecord{
			ID:         "test-id-2",
			Context:    "incoming context",
			Vector:     []float32{3.3, 4.4},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		}

		err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		// Verify insert
		row := provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'test-id-2'")
		var ctxStr string
		var status string
		if err := row.Scan(&ctxStr, &status); err != nil {
			t.Fatalf("failed to scan row: %v", err)
		}

		if ctxStr != "incoming context" {
			t.Errorf("expected context 'incoming context', got %s", ctxStr)
		}
		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
	})
}
