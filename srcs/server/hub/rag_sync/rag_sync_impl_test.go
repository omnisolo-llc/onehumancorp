package rag_sync

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = db.Exec(`
		INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES
		('1', 'test 1', '[0.1, 0.2]', 'pending'),
		('2', 'test 2', NULL, 'synced'),
		('3', 'test 3', '[0.5]', 'pending');
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	return db
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewRAGSyncService(db)

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	if records[0].ID != "1" && records[1].ID != "1" {
		t.Fatalf("missing record 1")
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{"1", "3"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(records))
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewRAGSyncService(db)

	now := time.Now().UTC()
	newRecords := []RAGSyncRecord{
		{
			ID:         "4",
			Context:    "test 4",
			Vector:     []float32{0.8, 0.9},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
		{
            ID:         "1", // Update existing
			Context:    "test 1 updated",
			Vector:     []float32{0.1, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
        },
	}

	err := svc.ProcessIncomingSync(context.Background(), newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT count(*) FROM autodream_memories WHERE id = '4'").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected record 4 to be inserted")
	}

	var content string
	err = db.QueryRow("SELECT content FROM autodream_memories WHERE id = '1'").Scan(&content)
	if err != nil || content != "test 1 updated" {
		t.Fatalf("expected record 1 to be updated")
	}
}
