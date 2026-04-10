package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncProvider(t *testing.T) {
	ctx := context.Background()

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Set up the exact schema required for testing
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	syncProvider := NewRAGSyncProvider(provider)

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('mem-1', 'context 1', X'00010203', 'pending'),
		       ('mem-2', 'context 2', X'00010204', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := syncProvider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "mem-1" {
		t.Errorf("expected mem-1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = syncProvider.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = syncProvider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed after mark: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	newRecs := []RAGSyncRecord{
		{
			ID:         "mem-3",
			Context:    "cloud context",
			Vector:     []byte{1, 2, 3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "mem-1",
			Context:    "updated context",
			Vector:     []byte{4, 5, 6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = syncProvider.ProcessIncomingSync(ctx, newRecs)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify mem-3 was inserted
	var ctxStr string
	row := provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem-3'")
	if err := row.Scan(&ctxStr); err != nil {
		t.Fatalf("failed to query mem-3: %v", err)
	}
	if ctxStr != "cloud context" {
		t.Errorf("expected cloud context, got %s", ctxStr)
	}

	// Verify mem-1 was updated
	row = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem-1'")
	if err := row.Scan(&ctxStr); err != nil {
		t.Fatalf("failed to query mem-1: %v", err)
	}
	if ctxStr != "updated context" {
		t.Errorf("expected updated context, got %s", ctxStr)
	}
}
