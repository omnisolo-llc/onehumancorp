package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	// Create an in-memory SQLite database connection
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}
	defer sqlDB.Close()

	// Use db.NewSqliteProvider to wrap the *sql.DB into db.Provider
	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()

	// Ensure the schema exists for testing. It includes the columns from the migration.
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert some pending records
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert record: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'ctx2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert record: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	pendingAfterMark, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(pendingAfterMark) != 1 {
		t.Errorf("expected 1 pending record after mark, got %d", len(pendingAfterMark))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "ctx3"},
		{ID: "1", Context: "ctx1_updated"},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	// Verify upsert via direct query
	row := provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '1'")
	var content, syncStatus string
	if err := row.Scan(&content, &syncStatus); err != nil {
		t.Fatalf("failed to scan verified record: %v", err)
	}

	if content != "ctx1_updated" {
		t.Errorf("expected content to be 'ctx1_updated', got '%s'", content)
	}
	if syncStatus != "synced" {
		t.Errorf("expected sync_status to be 'synced', got '%s'", syncStatus)
	}
}
