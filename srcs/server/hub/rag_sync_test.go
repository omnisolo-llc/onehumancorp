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
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	// Create required schema
	_, err = sqliteDB.Exec(`
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status) VALUES
		('1', 'pending content 1', 'pending'),
		('2', 'synced content', 'synced'),
		('3', 'pending content 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status) VALUES
		('1', 'pending content 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(provider)

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify update
	rows, err := provider.Query(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("expected row not found")
	}

	var status string
	if err := rows.Scan(&status); err != nil {
		t.Fatalf("failed to scan status: %v", err)
	}

	if status != "synced" {
		t.Fatalf("expected status 'synced', got %v", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewRAGSyncService(provider)

	records := []RAGSyncRecord{
		{
			ID:         "remote_1",
			Context:    "remote context",
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify insert
	rows, err := provider.Query(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'remote_1'")
	if err != nil {
		t.Fatalf("failed to query record: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("expected row not found")
	}

	var content, status string
	if err := rows.Scan(&content, &status); err != nil {
		t.Fatalf("failed to scan record: %v", err)
	}

	if content != "remote context" {
		t.Fatalf("expected 'remote context', got %v", content)
	}
	if status != "synced" {
		t.Fatalf("expected 'synced', got %v", status)
	}
}
