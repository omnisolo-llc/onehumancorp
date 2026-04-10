package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"database/sql"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in memory db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// create table
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
	memory_id        TEXT PRIMARY KEY,
	context          TEXT NOT NULL,
	vector_embedding BLOB,
	source_plugin    TEXT,
	created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     TIMESTAMPTZ NULL
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('test-1', 'test context 1', 'test-vector', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "test-1" {
		t.Fatalf("expected ID test-1, got %s", records[0].ID)
	}
	if string(records[0].Vector) != "test-vector" {
		t.Fatalf("expected vector test-vector, got %s", string(records[0].Vector))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('test-2', 'test context 2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"test-2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	var lastSyncAtStr sql.NullString
	err = provider.QueryRow(ctx, `SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'test-2'`).Scan(&status, &lastSyncAtStr)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Fatalf("expected status synced, got %s", status)
	}
	if !lastSyncAtStr.Valid || lastSyncAtStr.String == "" {
		t.Fatalf("expected last_sync_at to be set, got empty or invalid")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "test-3",
			Context:    "test context 3",
			Vector:     []byte("test-vector-3"),
			LastSyncAt: time.Now(),
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	var contextStr string
	err = provider.QueryRow(ctx, `SELECT sync_status, context FROM swarm_memory_embeddings WHERE memory_id = 'test-3'`).Scan(&status, &contextStr)
	if err != nil {
		t.Fatalf("failed to query record: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Fatalf("expected status synced, got %s", status)
	}
	if contextStr != "test context 3" {
		t.Fatalf("expected context 'test context 3', got %s", contextStr)
	}
}
