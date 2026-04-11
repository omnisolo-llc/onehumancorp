package hub

import (
	"context"
	"database/sql"
	"testing"
	"bytes"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRagSyncService(t *testing.T) {
	// Initialize an in-memory SQLite database
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}
	defer dbConn.Close()

	// Ensure pragmas for memory DB
	if _, err := dbConn.Exec("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;"); err != nil {
		t.Fatalf("failed to set PRAGMA: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	// Initialize the schema for tests. We use BLOB for vector in SQLite.
	schema := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     TIMESTAMPTZ NULL
	);
	`
	if _, err := dbConn.Exec(schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test 1: ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "mem_1",
			Context:    "Important insight 1",
			Vector:     []byte{1, 2, 3},
			SyncStatus: SyncStatusPending,
		},
		{
			ID:         "mem_2",
			Context:    "Important insight 2",
			Vector:     []byte{4, 5, 6},
			SyncStatus: SyncStatusPending,
		},
	}

	if err := service.ProcessIncomingSync(ctx, incoming); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insertion and vector
	var count int
	if err := dbConn.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count); err != nil {
		t.Fatalf("failed to count records: %v", err)
	}
	if count != 2 {
		t.Errorf("expected 2 records, got %d", count)
	}

	var vec []byte
	if err := dbConn.QueryRow("SELECT vector_embedding FROM swarm_memory_embeddings WHERE memory_id = 'mem_1'").Scan(&vec); err != nil {
		t.Fatalf("failed to get vector: %v", err)
	}
	if !bytes.Equal(vec, []byte{1,2,3}) {
		t.Errorf("expected vector [1,2,3], got %v", vec)
	}


	// Test 2: FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending syncs, got %d", len(pending))
	}
	if !bytes.Equal(pending[0].Vector, []byte{1,2,3}) {
		t.Errorf("expected vector [1,2,3], got %v", pending[0].Vector)
	}

	// Test 3: MarkSynced
	idsToMark := []string{"mem_1"}
	if err := service.MarkSynced(ctx, idsToMark); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify one is synced and one is pending
	var syncStatus string
	if err := dbConn.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem_1'").Scan(&syncStatus); err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if syncStatus != "synced" {
		t.Errorf("expected mem_1 to be synced, got %s", syncStatus)
	}

	if err := dbConn.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem_2'").Scan(&syncStatus); err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if syncStatus != "pending" {
		t.Errorf("expected mem_2 to be pending, got %s", syncStatus)
	}

	// Fetch again
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Errorf("expected 1 pending sync after mark, got %d", len(pendingAfter))
	}
	if pendingAfter[0].ID != "mem_2" {
		t.Errorf("expected pending sync to be mem_2, got %s", pendingAfter[0].ID)
	}
}
