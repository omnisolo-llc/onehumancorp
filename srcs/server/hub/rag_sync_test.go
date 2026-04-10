package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, func()) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()

	// Need to manually create the table since Goose migrations aren't executed in NewSqliteProvider
	createTableSQL := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT, -- SQLite doesn't have VECTOR type natively, store as TEXT
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMPTZ NULL
	);
	`
	_, err = provider.Exec(ctx, createTableSQL)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	return provider, func() {
		provider.Close()
	}
}

func TestDBService_Flow(t *testing.T) {
	provider, cleanup := setupTestDB(t)
	defer cleanup()

	service := NewDBService(provider)
	ctx := context.Background()

	vec1 := "[0.1, 0.2]"
	vec2 := "[0.3, 0.4]"

	// Insert test records
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ($1, $2, $3, $4)", "1", "test context 1", vec1, SyncStatusPending)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ($1, $2, $3, $4)", "2", "test context 2", vec2, SyncStatusPending)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify both are synced now
	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending records, got %d", len(pending))
	}

	// Verify sync status updated
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = $1", "1").Scan(&status)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status to be synced, got %s", status)
	}

	// Test ProcessIncomingSync
	vec3 := "[0.5, 0.6]"
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test context 3", Vector: &vec3, SyncStatus: SyncStatusPending},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming is synced
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = $1", "3").Scan(&status)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status to be synced, got %s", status)
	}
}
