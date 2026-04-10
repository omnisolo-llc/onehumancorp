package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/jackc/pgx/v5/pgxpool"
	_ "modernc.org/sqlite"
)

func TestStandaloneRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert record: %v", err)
	}

	svc := NewStandaloneRAGSyncService(db, nil)

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 || records[0].ID != "1" {
		t.Fatalf("Unexpected records: %+v", records)
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 0 {
		t.Fatalf("Expected 0 records, got %d", len(records))
	}

	err = svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
	if err == nil {
	    t.Fatalf("Expected error for ProcessIncomingSync in standalone mode")
	}
}

func TestCloudRAGSyncService(t *testing.T) {
    svc := NewCloudRAGSyncService(&pgxpool.Pool{}, nil)

    _, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
	    t.Fatalf("Expected error for FetchPendingSyncs in cloud mode")
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
	    t.Fatalf("Expected error for MarkSynced in cloud mode")
	}
}
