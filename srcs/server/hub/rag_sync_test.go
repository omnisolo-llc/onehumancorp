package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	_, err = dbConn.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err = dbConn.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES
			('1', 'Test context 1', x'010203', 'pending'),
			('2', 'Test context 2', x'040506', 'pending'),
			('3', 'Test context 3', x'070809', 'synced');
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// 1. FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	// 2. MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// Verify MarkSynced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 pending record after mark synced, got %d", len(records))
	}
	if records[0].ID != "2" {
		t.Fatalf("expected remaining pending record to be ID '2', got '%s'", records[0].ID)
	}

	// 3. ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{ID: "4", Context: "Incoming context", Vector: []byte{10, 11, 12}},
		{ID: "1", Context: "Updated context 1", Vector: []byte{13, 14, 15}}, // Test ON CONFLICT
	}
	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	err = dbConn.QueryRow("SELECT count(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("query error: %v", err)
	}
	// Initially 1 ('3') + 1 marked ('1') + 1 incoming new ('4') = 3. The incoming update for '1' keeps it at 3.
	if count != 3 {
		t.Fatalf("expected 3 synced records, got %d", count)
	}
}
