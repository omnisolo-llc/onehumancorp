package hub

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
		t.Fatalf("failed to open memory db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestFetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewSQLiteRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'synced'), ('3', 'test3', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(context.Background(), 2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
	if records[0].ID != "1" || records[1].ID != "3" {
		t.Fatalf("unexpected records: %+v", records)
	}
}

func TestMarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewSQLiteRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var status string
	err = db.QueryRow(`SELECT sync_status FROM autodream_memories WHERE id = '1'`).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Fatalf("expected status synced, got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewSQLiteRAGSyncService(db)

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	err = db.QueryRow(`SELECT COUNT(*) FROM autodream_memories`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 record, got %d", count)
	}
}
