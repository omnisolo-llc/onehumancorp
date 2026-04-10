package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // Important for SQLite tests
)

func TestDBRAGSyncServiceFlow(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}
	defer sqliteDB.Close()

	// Initialise schema
	_, err = sqliteDB.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = sqliteDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('rec1', 'context1', X'010203', 'pending'), ('rec2', 'context2', X'040506', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	service := NewDBRAGSyncService(provider)
	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("Expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	ids := []string{records[0].ID, records[1].ID}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify they are synced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "rec3", Context: "context3", Vector: []byte{7, 8, 9}},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming is stored
	row := sqliteDB.QueryRow("SELECT context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'rec3'")
	var contextStr, syncStatus string
	var vector []byte
	if err := row.Scan(&contextStr, &vector, &syncStatus); err != nil {
		t.Fatalf("failed to scan newly processed record: %v", err)
	}
	if contextStr != "context3" {
		t.Errorf("expected context3, got %s", contextStr)
	}
	if len(vector) != 3 || vector[0] != 7 || vector[1] != 8 || vector[2] != 9 {
		t.Errorf("unexpected vector value, got %v", vector)
	}
	if syncStatus != string(SyncStatusSynced) {
		t.Errorf("expected synced status, got %s", syncStatus)
	}
}
