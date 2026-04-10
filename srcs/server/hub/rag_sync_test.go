package hub

import (
	"context"
	"database/sql"
	"testing"

    "github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // Register SQLite
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

    // Create an in-memory SQLite database for testing
    sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }

	database := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	defer database.Close()

    // Initialize schema
	schema := `
	CREATE TABLE rag_memories (
		id TEXT PRIMARY KEY,
		context TEXT,
		vector TEXT,
		sync_status TEXT,
		last_sync_at TIMESTAMP
	);
	`
    _, err = database.Exec(ctx, schema)
    if err != nil {
         t.Fatalf("failed to init schema: %v", err)
    }


	svc := NewDefaultRAGSyncService(database)

	// 1. ProcessIncomingSync
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "test1", Context: "context 1", Vector: []float32{0.1, 0.2}},
	})
	if err != nil {
		t.Fatalf("expected no error processing incoming sync, got: %v", err)
	}

    // Insert a pending record manually for testing FetchPendingSyncs
    _, err = database.Exec(ctx, "INSERT INTO rag_memories (id, context, vector, sync_status) VALUES ('test2', 'context 2', '[0.3, 0.4]', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert pending record: %v", err)
    }

	// 2. FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error fetching pending syncs, got: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "test2" {
		t.Errorf("expected pending record ID 'test2', got '%s'", records[0].ID)
	}
    if records[0].SyncStatus != SyncStatusPending {
         t.Errorf("expected status 'pending', got '%v'", records[0].SyncStatus)
    }


	// 3. MarkSynced
	err = svc.MarkSynced(ctx, []string{"test2"})
	if err != nil {
		t.Fatalf("expected no error marking synced, got: %v", err)
	}

    // Verify it was marked synced
    records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error fetching pending syncs after mark synced, got: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after mark synced, got %d", len(records))
	}
}
