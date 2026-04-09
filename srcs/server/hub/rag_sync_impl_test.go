package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqldb)
	ctx := context.Background()

	// Run migration
	schema := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     DATETIME NULL
	);
	`
	if _, err := provider.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return provider
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert pending record
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'test context', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test record: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", records[0].ID)
	}
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert pending record
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'test context', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test record: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify sync status
	row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = '1'")
	var status string
	var lastSync sql.NullTime
	if err := row.Scan(&status, &lastSync); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
	if !lastSync.Valid {
		t.Errorf("expected last_sync_at to be valid")
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "synced context",
			SyncStatus: "synced",
			LastSyncAt: time.Now(),
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify upsert
	row := provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'")
	var contextStr, status string
	if err := row.Scan(&contextStr, &status); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if contextStr != "synced context" {
		t.Errorf("expected 'synced context', got '%s'", contextStr)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}
