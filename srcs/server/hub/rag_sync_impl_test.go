package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// We need to redefine NewTestProvider here, as it's defined in a _test.go file in the db package
// and cannot be imported by other packages.
import "database/sql"
import _ "modernc.org/sqlite"

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := d.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		d.Close()
	})

	return db.NewSqliteProvider(d)
}

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)

	service := NewRAGSyncService(provider)

	// Create some dummy records
	conn := provider

	// Create table first
	_, err := conn.Exec(ctx, `CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT,
		sync_status TEXT,
		organization_id TEXT,
		source_type TEXT,
		last_sync_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

    // Assuming migrations have run in the NewTestProvider
    _, err = conn.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status, organization_id, source_type) VALUES ('id1', 'content1', 'pending', 'org1', 'type1')")
    if err != nil {
        t.Fatalf("failed to insert test data: %v", err)
    }
    _, err = conn.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status, organization_id, source_type) VALUES ('id2', 'content2', 'synced', 'org1', 'type1')")
    if err != nil {
        t.Fatalf("failed to insert test data: %v", err)
    }

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		records, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(records) != 1 {
			t.Errorf("expected 1 record, got %d", len(records))
		}
		if records[0].ID != "id1" {
			t.Errorf("expected id1, got %s", records[0].ID)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := service.MarkSynced(ctx, []string{"id1"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		records, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(records) != 0 {
			t.Errorf("expected 0 records after MarkSynced, got %d", len(records))
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		incoming := []RAGSyncRecord{
			{
				ID:      "id3",
				Context: "content3",
			},
		}

		err := service.ProcessIncomingSync(ctx, incoming)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		// Verify it was inserted
		var count int
		err = conn.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = 'id3'").Scan(&count)
		if err != nil {
			t.Fatalf("unexpected error querying count: %v", err)
		}
		if count != 1 {
			t.Errorf("expected 1 record for id3, got %d", count)
		}
	})
}
