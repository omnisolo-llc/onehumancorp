package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open in-memory sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := hub.NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	now := time.Now()
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES
		('1', 'ctx1', X'0001', 'pending', NULL),
		('2', 'ctx2', X'0002', 'synced', ?),
		('3', 'ctx3', X'0003', 'pending', NULL)
	`, now)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	if records[0].ID != "1" && records[1].ID != "1" {
		t.Errorf("expected record 1")
	}
	if records[0].ID != "3" && records[1].ID != "3" {
		t.Errorf("expected record 3")
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := hub.NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES
		('1', 'ctx1', X'0001', 'pending', NULL),
		('2', 'ctx2', X'0002', 'pending', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	var lastSyncAt *time.Time
	row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = '1'")
	if err := row.Scan(&status, &lastSyncAt); err != nil {
		t.Fatalf("failed to query record 1: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set")
	}

	row = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '2'")
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to query record 2: %v", err)
	}
	if status != "pending" {
		t.Errorf("expected status 'pending', got '%s'", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := hub.NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data (simulating existing cloud data)
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ('1', 'old_ctx', X'0000', 'synced', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	now := time.Now()
	incoming := []hub.RAGSyncRecord{
		{ID: "1", Context: "new_ctx", Vector: []byte{0x01}, LastSyncAt: now},     // Should update
		{ID: "2", Context: "ctx2", Vector: []byte{0x02}, LastSyncAt: now},        // Should insert
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var ctxStr string
	row := provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'")
	if err := row.Scan(&ctxStr); err != nil {
		t.Fatalf("failed to query record 1: %v", err)
	}
	if ctxStr != "new_ctx" {
		t.Errorf("expected 'new_ctx', got '%s'", ctxStr)
	}

	row = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '2'")
	if err := row.Scan(&ctxStr); err != nil {
		t.Fatalf("failed to query record 2: %v", err)
	}
	if ctxStr != "ctx2" {
		t.Errorf("expected 'ctx2', got '%s'", ctxStr)
	}
}
