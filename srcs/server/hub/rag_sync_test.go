package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func setupTestDB(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	// Set up the autodream_memories table schema for the tests
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	return provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	service, err := hub.NewHybridRAGSyncService(provider)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES
			('mem1', 'context 1', 'pending'),
			('mem2', 'context 2', 'pending'),
			('mem3', 'context 3', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	foundMem1 := false
	for _, rec := range records {
		if rec.ID == "mem1" {
			foundMem1 = true
			if rec.Context != "context 1" {
				t.Errorf("expected context 'context 1', got '%s'", rec.Context)
			}
			if rec.SyncStatus != hub.SyncStatusPending {
				t.Errorf("expected sync status pending, got '%s'", rec.SyncStatus)
			}
		}
	}

	if !foundMem1 {
		t.Errorf("expected to find mem1 in pending syncs")
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	service, err := hub.NewHybridRAGSyncService(provider)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('mem1', 'context 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error fetching pending syncs, got %v", err)
	}

	if len(records) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Verify the database record directly
	var status string
	var lastSyncAt *time.Time
	err = provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = 'mem1'").Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("expected no error querying db directly, got %v", err)
	}

	if status != string(hub.SyncStatusSynced) {
		t.Errorf("expected status to be synced, got %s", status)
	}

	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set, but it was nil")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	service, err := hub.NewHybridRAGSyncService(provider)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	// Insert an existing record to test the ON CONFLICT behavior
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('mem_existing', 'old context', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records := []hub.RAGSyncRecord{
		{
			ID:      "mem_existing",
			Context: "new context",
		},
		{
			ID:      "mem_new",
			Context: "newly synced context",
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error processing incoming sync, got %v", err)
	}

	// Verify mem_existing was updated
	var existingContext, existingStatus string
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'mem_existing'").Scan(&existingContext, &existingStatus)
	if err != nil {
		t.Fatalf("expected no error querying mem_existing, got %v", err)
	}

	if existingContext != "new context" {
		t.Errorf("expected mem_existing context to be updated, got '%s'", existingContext)
	}
	if existingStatus != string(hub.SyncStatusSynced) {
		t.Errorf("expected mem_existing status to be synced, got '%s'", existingStatus)
	}

	// Verify mem_new was inserted
	var newContext, newStatus string
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'mem_new'").Scan(&newContext, &newStatus)
	if err != nil {
		t.Fatalf("expected no error querying mem_new, got %v", err)
	}

	if newContext != "newly synced context" {
		t.Errorf("expected mem_new context to be correct, got '%s'", newContext)
	}
	if newStatus != string(hub.SyncStatusSynced) {
		t.Errorf("expected mem_new status to be synced, got '%s'", newStatus)
	}
}
