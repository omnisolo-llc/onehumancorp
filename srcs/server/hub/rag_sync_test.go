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
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	// Create table schema
	query := `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ
		);
	`
	if _, err := sqliteDB.Exec(query); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Seed data
	query := `
		INSERT INTO autodream_memories (id, content, sync_status) VALUES
		('1', 'test 1', 'pending'),
		('2', 'test 2', 'synced'),
		('3', 'test 3', 'pending');
	`
	if _, err := provider.Exec(ctx, query); err != nil {
		t.Fatalf("failed to seed data: %v", err)
	}

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		pending, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(pending) != 2 {
			t.Errorf("expected 2 pending records, got %d", len(pending))
		}
		if (pending[0].ID != "1" && pending[0].ID != "3") || (pending[1].ID != "1" && pending[1].ID != "3") {
			t.Errorf("unexpected pending records: %+v", pending)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := svc.MarkSynced(ctx, []string{"1"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		pending, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(pending) != 1 {
			t.Errorf("expected 1 pending record after marking one synced, got %d", len(pending))
		}
		if pending[0].ID != "3" {
			t.Errorf("expected remaining pending record to be ID 3")
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		now := time.Now()
		err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
			{ID: "4", Context: "test 4", SyncStatus: SyncStatusSynced, LastSyncAt: now},
		})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		// Verify using query
		row := provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '4'")
		var content, syncStatus string
		if err := row.Scan(&content, &syncStatus); err != nil {
			t.Fatalf("failed to scan verified record: %v", err)
		}
		if content != "test 4" || syncStatus != "synced" {
			t.Errorf("unexpected record values: %s, %s", content, syncStatus)
		}
	})
}
