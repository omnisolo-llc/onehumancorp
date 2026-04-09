package hub

import (
	"context"
	"database/sql"
	"reflect"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	createTableQuery := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestFetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewDBRAGSyncService(db)

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'test_context', '[0.1, 0.2, 0.3]', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", records[0].ID)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected status 'pending', got '%s'", records[0].SyncStatus)
	}
	expectedVector := []float32{0.1, 0.2, 0.3}
	if !reflect.DeepEqual(records[0].Vector, expectedVector) {
		t.Errorf("expected vector %v, got %v", expectedVector, records[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewDBRAGSyncService(db)

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test_context', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var status string
	err = db.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewDBRAGSyncService(db)

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "new_context",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var content string
	var status string
	var embedding string
	err = db.QueryRow("SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '1'").Scan(&content, &embedding, &status)
	if err != nil {
		t.Fatalf("failed to query inserted record: %v", err)
	}

	if content != "new_context" {
		t.Errorf("expected content 'new_context', got '%s'", content)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
	expectedEmbedding := "[0.4,0.5,0.6]"
	if embedding != expectedEmbedding {
		t.Errorf("expected embedding '%s', got '%s'", expectedEmbedding, embedding)
	}
}
