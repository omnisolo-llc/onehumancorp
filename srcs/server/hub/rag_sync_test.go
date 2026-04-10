package hub

import (
	"context"
	"database/sql"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	testProvider := db.NewSqliteProvider(sqliteDB)

	_, err = sqliteDB.Exec(`
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			source_mission_id TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	dbWrapper := &db.DB{Provider: testProvider}

	svc, err := NewRAGSyncService(dbWrapper)
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:      "test-id-1",
			Context: "test content 1",
			Vector:  []byte{1, 2, 3},
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	// Because ProcessIncomingSync sets sync_status to 'synced', let's manually insert a pending record
	_, err = dbWrapper.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-pending', 'content', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"test-pending"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs after mark: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pendingAfter))
	}
}
