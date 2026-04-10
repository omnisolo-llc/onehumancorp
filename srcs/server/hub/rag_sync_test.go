package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService_Local(t *testing.T) {
	// Initialize in-memory SQLite provider for testing
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Create table
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TEXT,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert test data
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'context1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'context2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('3', 'context3', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 pending record after marking one synced, got %d", len(records))
	}
	if records[0].ID != "2" {
		t.Errorf("expected record ID 2, got %s", records[0].ID)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "4",
			Context:    "context4",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "1", // Update existing
			Context:    "context1_updated",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming syncs
	var count int
	err = sqlDB.QueryRow(`SELECT COUNT(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 3 { // 3, 1(updated), 4
		t.Errorf("expected 3 synced records, got %d", count)
	}

	var updatedCtx string
	err = sqlDB.QueryRow(`SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'`).Scan(&updatedCtx)
	if err != nil {
		t.Fatalf("failed to query updated context: %v", err)
	}
	if updatedCtx != "context1_updated" {
		t.Errorf("expected updated context 'context1_updated', got '%s'", updatedCtx)
	}
}
