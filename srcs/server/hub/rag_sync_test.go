package hub

import (
	"context"
	"database/sql"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestHybridRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	dbProvider := db.NewSqliteProvider(sqliteDB)
	defer dbProvider.Close()

	// create table
	_, err = dbProvider.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT NOT NULL, vector_embedding BYTEA, source_plugin TEXT, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, sync_status VARCHAR(50) DEFAULT 'pending', last_sync_at TIMESTAMP NULL)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewHybridRAGSyncService(dbProvider)

	// insert mock data
	_, err = dbProvider.Exec(context.Background(), "INSERT INTO swarm_memory_embeddings (memory_id, context) VALUES ($1, $2)", "1", "test context 1")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = dbProvider.Exec(context.Background(), "INSERT INTO swarm_memory_embeddings (memory_id, context) VALUES ($1, $2)", "2", "test context 2")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	ctx := context.Background()
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "2" {
		t.Errorf("expected record 2, got %s", records[0].ID)
	}
}
