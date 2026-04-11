package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	// Create table
	_, err = dbConn.Exec(`
		DROP TABLE IF EXISTS autodream_memories;
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

	return db.NewSqliteProvider(dbConn)
}

func TestDatabaseRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test content 2', 'synced')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('3', 'test content 3', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewDatabaseRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestDatabaseRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'content 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewDatabaseRAGSyncService(provider)

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'")
	var status string
	var lastSync *time.Time
	if err := row.Scan(&status, &lastSync); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
	if lastSync == nil {
		t.Errorf("expected last_sync_at to be set")
	}
}

func TestDatabaseRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'old content', 'synced')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewDatabaseRAGSyncService(provider)

	records := []RAGSyncRecord{
		{ID: "1", Context: "new content", Vector: []float32{1.1, 2.2}},
		{ID: "2", Context: "new record", Vector: []float32{3.3, 4.4}},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT id, content FROM autodream_memories ORDER BY id ASC")
	if err != nil {
		t.Fatalf("failed to query: %v", err)
	}
	defer rows.Close()

	var id, content string

	if !rows.Next() {
		t.Fatalf("expected row")
	}
	rows.Scan(&id, &content)
	if id != "1" || content != "new content" {
		t.Errorf("record 1 not updated correctly, got id=%s, content=%s", id, content)
	}

	if !rows.Next() {
		t.Fatalf("expected row")
	}
	rows.Scan(&id, &content)
	if id != "2" || content != "new record" {
		t.Errorf("record 2 not inserted correctly, got id=%s, content=%s", id, content)
	}
}
