package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite" // Use the modernc sqlite driver for tests
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}

	// Create table
	_, err = db.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at TEXT DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TEXT NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewRAGSyncService(db)

	// Insert test data
	_, err := db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES
			('1', 'ctx1', 'pending'),
			('2', 'ctx2', 'synced'),
			('3', 'ctx3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	ctx := context.Background()
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	// Verify records
	ids := map[string]bool{}
	for _, r := range records {
		ids[r.ID] = true
		if r.SyncStatus != SyncStatusPending {
			t.Errorf("expected status 'pending', got '%s'", r.SyncStatus)
		}
	}
	if !ids["1"] || !ids["3"] {
		t.Errorf("expected records 1 and 3, got: %v", ids)
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewRAGSyncService(db)

	_, err := db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('1', 'ctx1', 'pending'), ('2', 'ctx2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	ctx := context.Background()
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify in DB
	var status1, status2 string
	_ = db.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status1)
	_ = db.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '2'").Scan(&status2)

	if status1 != "synced" {
		t.Errorf("expected record 1 to be synced, got %s", status1)
	}
	if status2 != "pending" {
		t.Errorf("expected record 2 to be pending, got %s", status2)
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewRAGSyncService(db)

	ctx := context.Background()
	records := []RAGSyncRecord{
		{ID: "1", Context: "ctx1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
		{ID: "2", Context: "ctx2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify in DB
	var count int
	_ = db.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if count != 2 {
		t.Errorf("expected 2 records, got %d", count)
	}

	var ctx1 string
	_ = db.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&ctx1)
	if ctx1 != "ctx1" {
		t.Errorf("expected ctx1, got %s", ctx1)
	}

	// Update existing record
	records[0].Context = "ctx1_updated"
	err = svc.ProcessIncomingSync(ctx, records[:1])
	if err != nil {
		t.Fatalf("unexpected error during update: %v", err)
	}

	_ = db.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&ctx1)
	if ctx1 != "ctx1_updated" {
		t.Errorf("expected ctx1_updated, got %s", ctx1)
	}
}
