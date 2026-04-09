package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	_, err = sqldb.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqldb)
}

func TestRAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)

	ctx := context.Background()

	// Insert initial data directly
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ($1, $2, $3)", "test1", "context 1", string(SyncStatusPending))
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "test1" {
		t.Errorf("expected ID 'test1', got '%s'", records[0].ID)
	}

	err = svc.MarkSynced(ctx, []string{"test1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs after mark: %v", err)
	}

	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after mark, got %d", len(records))
	}

	incoming := []RAGSyncRecord{
		{
			ID:         "test2",
			Context:    "context 2",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = $1", "test2")
	var status string
	err = row.Scan(&status)
	if err != nil {
		t.Fatalf("failed to scan sync_status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}
