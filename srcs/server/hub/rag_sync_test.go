package hub

import (
	"context"
	"database/sql"
	"testing"
	"bytes"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	database, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := database.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		database.Close()
	})

	return db.NewSqliteProvider(database)
}

func TestHubRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Create table schema manually for the test
	_, err := provider.Exec(ctx, `
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

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES
			('mem1', 'ctx1', ?, 'pending', NULL),
			('mem2', 'ctx2', ?, 'synced', CURRENT_TIMESTAMP),
			('mem3', 'ctx3', ?, 'pending', NULL)
	`, []byte{1, 2, 3}, []byte{4, 5, 6}, []byte{7, 8, 9})
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewHubRAGSyncService(provider)

	// Fetch 1 record
	records, err := service.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Fatalf("expected status pending, got %v", records[0].SyncStatus)
	}
	if !bytes.Equal(records[0].Vector, []byte{1, 2, 3}) {
		t.Fatalf("expected vector {1,2,3}, got %v", records[0].Vector)
	}

	// Fetch remaining records (limit 10)
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
}

func TestHubRAGSyncService_MarkSynced(t *testing.T) {
	provider := NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Create table schema manually for the test
	_, err := provider.Exec(ctx, `
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

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
		VALUES
			('mem1', 'ctx1', 'pending', NULL),
			('mem2', 'ctx2', 'pending', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewHubRAGSyncService(provider)

	err = service.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify update
	rows, err := provider.Query(ctx, `SELECT memory_id, sync_status FROM swarm_memory_embeddings WHERE memory_id IN ('mem1', 'mem2') ORDER BY memory_id`)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	defer rows.Close()

	var id, status string
	rows.Next()
	rows.Scan(&id, &status)
	if id != "mem1" || status != "synced" {
		t.Fatalf("expected mem1 to be synced, got %s: %s", id, status)
	}

	rows.Next()
	rows.Scan(&id, &status)
	if id != "mem2" || status != "pending" {
		t.Fatalf("expected mem2 to be pending, got %s: %s", id, status)
	}
}

func TestHubRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Create table schema manually for the test
	_, err := provider.Exec(ctx, `
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

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES
			('mem1', 'ctx1_old', ?, 'synced', CURRENT_TIMESTAMP)
	`, []byte{1, 1, 1})
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewHubRAGSyncService(provider)

	records := []RAGSyncRecord{
		{ID: "mem1", Context: "ctx1_new", Vector: []byte{2, 2, 2}, SyncStatus: SyncStatusPending},
		{ID: "mem2", Context: "ctx2_new", Vector: []byte{3, 3, 3}, SyncStatus: SyncStatusPending},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify updates and inserts
	rows, err := provider.Query(ctx, `SELECT memory_id, context, vector_embedding, sync_status FROM swarm_memory_embeddings ORDER BY memory_id`)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	defer rows.Close()

	var id, contextData, status string
	var vector []byte

	rows.Next()
	rows.Scan(&id, &contextData, &vector, &status)
	if id != "mem1" || contextData != "ctx1_new" || status != "synced" {
		t.Fatalf("expected mem1 ctx1_new synced, got %s %s %s", id, contextData, status)
	}
	if !bytes.Equal(vector, []byte{2, 2, 2}) {
		t.Fatalf("expected vector {2,2,2}, got %v", vector)
	}

	rows.Next()
	rows.Scan(&id, &contextData, &vector, &status)
	if id != "mem2" || contextData != "ctx2_new" || status != "synced" {
		t.Fatalf("expected mem2 ctx2_new synced, got %s %s %s", id, contextData, status)
	}
	if !bytes.Equal(vector, []byte{3, 3, 3}) {
		t.Fatalf("expected vector {3,3,3}, got %v", vector)
	}
}
