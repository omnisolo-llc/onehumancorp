package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// Create a local test provider since NewTestProvider is in a test file in db package
func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)

	// Ensure the table exists in the test DB
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Insert some pending records
	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert records: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify they are synced
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records, got %d", count)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "test3",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "1", // Update existing
			Context:    "test1-updated",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify
	var content string
	err = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '1'").Scan(&content)
	if err != nil {
		t.Fatalf("failed to query updated content: %v", err)
	}
	if content != "test1-updated" {
		t.Fatalf("expected test1-updated, got %s", content)
	}
}
