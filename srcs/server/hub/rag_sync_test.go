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
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite memory db: %v", err)
	}

	provider := db.NewSqliteProvider(d)
	ctx := context.Background()

	// Setup schema
	schema := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at DATETIME NULL
	);
	`
	if _, err := provider.Exec(ctx, schema); err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	return provider
}

func TestRagSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRagSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('id1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('id2', 'ctx2', 'synced')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs returned error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "id1" {
		t.Errorf("Expected record ID id1, got %s", records[0].ID)
	}
}

func TestRagSyncServiceImpl_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRagSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('id1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"id1"})
	if err != nil {
		t.Fatalf("MarkSynced returned error: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'id1'")
	if err != nil {
		t.Fatalf("Failed to query: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("Expected row not found")
	}

	var status string
	if err := rows.Scan(&status); err != nil {
		t.Fatalf("Failed to scan: %v", err)
	}

	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}

func TestRagSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRagSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "id1", Context: "incoming ctx", Vector: "vec", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync returned error: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'id1'")
	if err != nil {
		t.Fatalf("Failed to query: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("Expected row not found")
	}

	var contextStr, status string
	if err := rows.Scan(&contextStr, &status); err != nil {
		t.Fatalf("Failed to scan: %v", err)
	}

	if contextStr != "incoming ctx" {
		t.Errorf("Expected context 'incoming ctx', got '%s'", contextStr)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}
