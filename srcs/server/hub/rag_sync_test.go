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
	// Create temp file
	f, err := os.CreateTemp("", "test_db_*.sqlite")
	if err != nil {
		t.Fatalf("Failed to create temp db file: %v", err)
	}
	dbPath := f.Name()
	f.Close()

	sqlDB, err := sql.Open("sqlite", dbPath)
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	// create tables
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE swarm_memory_embeddings (
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
		provider.Close()
		os.Remove(dbPath)
		t.Fatalf("Failed to create table: %v", err)
	}

	cleanup := func() {
		provider.Close()
		os.Remove(dbPath)
	}
	return provider, cleanup
}

func TestRAGSyncService(t *testing.T) {
	provider, cleanup := setupTestDB(t)
	defer cleanup()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('test1', 'context 1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('test2', 'context 2', 'synced')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "test1" {
		t.Fatalf("Expected record ID test1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"test1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "test3", Context: "context 3"},
		{ID: "test1", Context: "context 1 updated"},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT count(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("Failed to scan count: %v", err)
	}
	if count != 3 {
		t.Fatalf("Expected 3 synced records, got %d", count)
	}
}
