package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/google/uuid"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
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

	return db
}

func TestSQLRAGSyncService_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewSQLRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'test1', 'vec1', 'pending'), ('2', 'test2', 'vec2', 'synced'), ('3', 'test3', 'vec3', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
}

func TestSQLRAGSyncService_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewSQLRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'test1', 'vec1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status string
	var lastSyncAt sql.NullTime
	err = db.QueryRow(`SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'`).Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != "synced" {
		t.Fatalf("expected status 'synced', got %s", status)
	}

	if !lastSyncAt.Valid {
		t.Fatalf("expected last_sync_at to be valid")
	}
}

func TestSQLRAGSyncService_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewSQLRAGSyncService(db)

	id1 := uuid.New().String()

	records := []RAGSyncRecord{
		{ID: id1, Context: "new_content_1", Vector: "vec1"},
		{ID: uuid.New().String(), Context: "new_content_2", Vector: "vec2"},
	}

	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = db.QueryRow(`SELECT COUNT(*) FROM autodream_memories`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 2 {
		t.Fatalf("expected 2 records inserted, got %d", count)
	}

	// Test update existing
	records[0].Context = "updated_content_1"
	records[0].Vector = "vec4"
	err = svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{records[0]})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var content string
	var vector string
	err = db.QueryRow(`SELECT content, embedding FROM autodream_memories WHERE id = ?`, id1).Scan(&content, &vector)
	if err != nil {
		t.Fatalf("failed to query content: %v", err)
	}

	if vector != "vec4" {
		t.Fatalf("expected 'vec4', got %s", vector)
	}

	if content != "updated_content_1" {
		t.Fatalf("expected 'updated_content_1', got %s", content)
	}
}
