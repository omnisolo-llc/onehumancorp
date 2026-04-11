package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, func()) {
	f, err := os.CreateTemp("", "rag_sync_test_*.db")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}

	sqliteDB, err := sql.Open("sqlite", f.Name())
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider, func() {
		provider.Close()
		os.Remove(f.Name())
	}
}

func TestRAGSyncService(t *testing.T) {
	provider, cleanup := setupTestDB(t)
	defer cleanup()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ($1, $2, $3, $4)", "1", "test context", []byte{1, 2, 3}, "pending")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" || records[0].Context != "test context" {
		t.Errorf("unexpected record data: %+v", records[0])
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching after marked synced: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test process incoming sync
	incoming := []RAGSyncRecord{
		{ID: "2", Context: "new context", Vector: []byte{4, 5, 6}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}
}
