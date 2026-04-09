package hub

import (
	"context"
	"testing"
	"time"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	// Set DSN explicitly for tests
	// using file::memory:?mode=memory
	os.Setenv("OHC_DB_DSN", "sqlite://file::memory:?mode=memory")
	database, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("Failed to create test db: %v", err)
	}

	// Clean table manually before use in test
	_, err = database.Exec(context.Background(), `
		DROP TABLE IF EXISTS swarm_memory_embeddings;
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	return database
}

func TestFetchPendingSyncs(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	database := setupTestDB(t)
	defer database.Provider.Close()

	service := NewDefaultRAGSyncService(database)
	ctx := context.Background()

	_, err := database.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "1", "ctx1", "pending")
	if err != nil { t.Fatalf("err: %v", err) }
	_, err = database.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "2", "ctx2", "synced")
	if err != nil { t.Fatalf("err: %v", err) }
	_, err = database.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "3", "ctx3", "pending")
	if err != nil { t.Fatalf("err: %v", err) }

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	database := setupTestDB(t)
	defer database.Provider.Close()

	service := NewDefaultRAGSyncService(database)
	ctx := context.Background()

	_, err := database.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "1", "ctx1", "pending")
	if err != nil { t.Fatalf("err: %v", err) }

	ids := []string{"1"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	err = database.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = ?", "1").Scan(&status)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected synced, got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	database := setupTestDB(t)
	defer database.Provider.Close()

	service := NewDefaultRAGSyncService(database)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "1", Context: "context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var count int
	err = database.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record, got %d", count)
	}
}
