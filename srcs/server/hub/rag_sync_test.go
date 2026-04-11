package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	ctx := context.Background()

	// Setup schema
	query := `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`
	_, err = provider.Exec(ctx, query)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES
			('rec1', 'Context 1', 'pending'),
			('rec2', 'Context 2', 'synced'),
			('rec3', 'Context 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	if records[0].SyncStatus != SyncStatusPending || records[1].SyncStatus != SyncStatusPending {
		t.Errorf("expected status pending")
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('rec1', 'Context 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"rec1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = 'rec1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status synced, got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "in1", Context: "Incoming 1"},
		{ID: "in2", Context: "Incoming 2"},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 2 {
		t.Errorf("expected 2 records synced, got %d", count)
	}
}
