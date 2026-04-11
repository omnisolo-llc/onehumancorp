package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	f, err := os.CreateTemp("", "rag_sync_test_*.db")
	if err != nil {
		t.Fatalf("failed to create temp db file: %v", err)
	}
	f.Close()
	defer os.Remove(f.Name())

	sqliteDB, err := sql.Open("sqlite", "file:"+f.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	dbProvider := db.NewSqliteProvider(sqliteDB)

	// Manual schema setup for test
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      TEXT DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	svc := NewRAGSyncService(dbProvider)

	_, err = dbProvider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('test-id', 'test context', x'010203')`)
	if err != nil {
		t.Fatalf("insert error: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != SyncStatusInProgress {
		t.Fatalf("expected status to be updated to in_progress, got %s", records[0].SyncStatus)
	}

	err = svc.MarkSynced(ctx, []string{"test-id"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Process new sync
	newRecords := []RAGSyncRecord{
		{ID: "test-id-2", Context: "new context", Vector: []byte{4, 5, 6}},
	}
	err = svc.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify Upsert created the record
	row := dbProvider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'test-id-2'")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to verify upsert: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Fatalf("expected upserted record to have status synced, got %s", status)
	}
}
