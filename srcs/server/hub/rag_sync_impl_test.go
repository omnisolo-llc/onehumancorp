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
		t.Fatalf("Failed to open db: %v", err)
	}

	createTableQuery := `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`
	if _, err := db.Exec(createTableQuery); err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}
	_, err = db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test content 2', 'synced')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	svc := NewRAGSyncService(db)
	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("Expected ID '1', got '%s'", records[0].ID)
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	svc := NewRAGSyncService(db)
	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	var lastSync sql.NullTime
	err = db.QueryRow("SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("Failed to query: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
	if !lastSync.Valid {
		t.Error("Expected last_sync_at to be set")
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewRAGSyncService(db)

	now := time.Now()
	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "incoming content",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var content string
	var status string
	err = db.QueryRow("SELECT content, sync_status FROM autodream_memories WHERE id = '1'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("Failed to query: %v", err)
	}

	if content != "incoming content" {
		t.Errorf("Expected content 'incoming content', got '%s'", content)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}
