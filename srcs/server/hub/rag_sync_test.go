package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	_, err = db.Exec(`CREATE TABLE consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		source_type TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return db
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	_, err := db.Exec(`INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('1', 'org', 'data1', 'type1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = db.Exec(`INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('2', 'org', 'data2', 'type1', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	svc := NewConcreteRAGSyncService(db)
	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("failed to fetch: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	_, err := db.Exec(`INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('1', 'org', 'data1', 'type1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	svc := NewConcreteRAGSyncService(db)
	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	var status string
	err = db.QueryRow(`SELECT sync_status FROM consolidated_memory WHERE id = '1'`).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status synced, got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	svc := NewConcreteRAGSyncService(db)
	records := []RAGSyncRecord{
		{ID: "1", Context: "new data", SyncStatus: "synced"},
	}
	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("failed to process: %v", err)
	}

	var content string
	var status string
	err = db.QueryRow(`SELECT content, sync_status FROM consolidated_memory WHERE id = '1'`).Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query: %v", err)
	}
	if content != "new data" {
		t.Errorf("expected content 'new data', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	// Test Update
	records[0].Context = "updated data"
	err = svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("failed to update: %v", err)
	}

	err = db.QueryRow(`SELECT content FROM consolidated_memory WHERE id = '1'`).Scan(&content)
	if err != nil {
		t.Fatalf("failed to query: %v", err)
	}
	if content != "updated data" {
		t.Errorf("expected content 'updated data', got '%s'", content)
	}
}
