package hub

import (
	"context"
	"testing"
	"time"
	"encoding/json"
    "database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncServiceImpl(t *testing.T) {
	// Initialize in-memory SQLite for testing
	ctx := context.Background()

	provider := NewTestProvider(t)
	defer provider.Close()

	// Execute migrations manually for testing since NewSqliteProviderForTest doesn't do it automatically
	_, err := provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Insert some pending data
	vec := []float32{0.1, 0.2, 0.3}
	vecBytes, _ := json.Marshal(vec)
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('test-1', 'pending context', $1, 'pending')
	`, string(vecBytes))
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// 1. Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-1" || records[0].SyncStatus != SyncStatusPending {
		t.Fatalf("unexpected record data: %+v", records[0])
	}

	// 2. Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify sync status updated
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = 'test-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected status 'synced', got %s", status)
	}

	// 3. Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID: "test-2",
			Context: "cloud context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID: "test-1", // Update existing
			Context: "updated pending context from cloud",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records, got %d", count)
	}
}
