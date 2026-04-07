package hub

import (
	"context"
	"database/sql"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// Add test logic to test the concrete DefaultRAGSyncService against an SQLite provider.

func TestDefaultRAGSyncService(t *testing.T) {
	tempDir := t.TempDir()
	dbPath := filepath.Join(tempDir, "test.db")

	sqlDB, err := sql.Open("sqlite", dbPath)
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()

	// Create table for tests (since we skip the migrations for unit test context here)
	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewDefaultRAGSyncService(provider)

	// Seed data
	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'memory 1', '[0.1, 0.2]', 'pending')`)
	if err != nil {
		t.Fatalf("failed to seed data: %v", err)
	}

	// Fetch Pending Syncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error fetching pending syncs, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected record ID '1', got '%s'", records[0].ID)
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 || records[0].Vector[1] != 0.2 {
		t.Errorf("expected vector [0.1, 0.2], got %v", records[0].Vector)
	}

	// Mark Synced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error marking synced, got %v", err)
	}

	// Verify Mark Synced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error fetching pending syncs, got %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after sync, got %d", len(records))
	}

	// Process Incoming Sync
	incoming := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming memory 2",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error processing incoming sync, got %v", err)
	}

	// Verify Incoming
	var content string
	var status string
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '2'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("expected no error querying incoming sync, got %v", err)
	}
	if content != "incoming memory 2" {
		t.Errorf("expected content 'incoming memory 2', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}
