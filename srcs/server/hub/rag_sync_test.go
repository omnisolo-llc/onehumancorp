package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	_, err = sqlDB.Exec(`
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

	return &db.DB{Provider: db.NewSqliteProvider(sqlDB)}
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	database := setupTestDB(t)
	svc := NewDefaultRAGSyncService(database)

	ctx := context.Background()

	// Insert some test data
	_, err := database.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test content 1', 'pending'), ('2', 'test content 2', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Fatalf("expected record ID 1, got %s", records[0].ID)
	}
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	database := setupTestDB(t)
	svc := NewDefaultRAGSyncService(database)

	ctx := context.Background()

	// Insert test data
	_, err := database.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test content 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	var lastSyncAt *time.Time
	row := database.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'")
	err = row.Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("failed to fetch row: %v", err)
	}

	if status != "synced" {
		t.Fatalf("expected status synced, got %s", status)
	}
	if lastSyncAt == nil {
		t.Fatalf("expected last_sync_at to be set")
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	database := setupTestDB(t)
	svc := NewDefaultRAGSyncService(database)

	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "test content",
			Vector:     []float32{1.0, 2.0},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var content string
	var status string
	var embedding string
	row := database.QueryRow(ctx, "SELECT content, sync_status, embedding FROM autodream_memories WHERE id = '1'")
	err = row.Scan(&content, &status, &embedding)
	if err != nil {
		t.Fatalf("failed to fetch row: %v", err)
	}

	if content != "test content" {
		t.Fatalf("expected test content, got %s", content)
	}
	if status != "synced" {
		t.Fatalf("expected synced status, got %s", status)
	}
	if embedding != "[1,2]" {
		t.Fatalf("expected embedding [1,2], got %s", embedding)
	}
}
