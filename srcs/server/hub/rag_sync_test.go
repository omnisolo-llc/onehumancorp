package hub

import (
	"context"
	"database/sql"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestSQLRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize test database directly since NewTestProvider is in a test package
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	t.Cleanup(func() { d.Close() })

	prov := db.NewSqliteProvider(d)

	// Since we need autodream_memories, let's create the schema here for the test.
	_, err = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	svc := NewRAGSyncService(prov)

	// 1. Insert some pending syncs
	_, err = prov.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'ctx 1', 'pending'), ('2', 'ctx 2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	// 2. Fetch Pending Syncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}

	// 3. Process Incoming Syncs (Upsert)
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "ctx 3"},
		{ID: "1", Context: "ctx 1 updated"},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming were inserted/updated and marked synced
	var count int
	err = prov.QueryRow(ctx, `SELECT count(*) FROM autodream_memories WHERE sync_status = 'synced'`).Scan(&count)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if count != 2 {
		t.Errorf("expected 2 synced records from incoming, got %d", count)
	}

	var content string
	err = prov.QueryRow(ctx, `SELECT content FROM autodream_memories WHERE id = '1'`).Scan(&content)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if content != "ctx 1 updated" {
		t.Errorf("expected 'ctx 1 updated', got '%s'", content)
	}

	// 4. Mark Synced
	err = svc.MarkSynced(ctx, []string{"2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	err = prov.QueryRow(ctx, `SELECT count(*) FROM autodream_memories WHERE sync_status = 'synced'`).Scan(&count)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if count != 3 {
		t.Errorf("expected 3 total synced records, got %d", count)
	}
}
