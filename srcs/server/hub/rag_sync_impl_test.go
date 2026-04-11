package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (*sql.DB, func()) {
	f, err := os.CreateTemp("", "testdb-*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}

	db, err := sql.Open("sqlite", f.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = db.Exec(`CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	cleanup := func() {
		db.Close()
		os.Remove(f.Name())
	}
	return db, cleanup
}

func TestSQLRAGSyncService(t *testing.T) {
	db, cleanup := setupTestDB(t)
	defer cleanup()

	svc := NewSQLRAGSyncService(db, true) // true for sqlite
	ctx := context.Background()

	err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "m1", Context: "test 1", Vector: []byte{1, 2}, SyncStatus: "synced"},
	})

	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query count: %v", err)
	}
	if count != 1 {
		t.Fatalf("Expected 1 row, got %d", count)
	}

	_, err = db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('m2', 'test 2', ?, 'pending')", []byte{3, 4})
	if err != nil {
		t.Fatalf("Failed to insert pending row: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending row, got %d", len(pending))
	}
	if pending[0].ID != "m2" {
		t.Fatalf("Expected id m2, got %s", pending[0].ID)
	}

	err = svc.MarkSynced(ctx, []string{"m2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("Expected 0 pending row, got %d", len(pending))
	}
}
