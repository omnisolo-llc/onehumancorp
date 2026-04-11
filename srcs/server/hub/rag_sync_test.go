package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // In-memory sqlite driver
)

func TestRagSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}
	defer sqliteDB.Close()

	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	ctx := context.Background()

	// Setup schema
	setupQuery := `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	if _, err := dbWrapper.Exec(ctx, setupQuery); err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	insertQuery := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('test-1', 'test content 1', '[0.1, 0.2, 0.3]', 'pending'),
		       ('test-2', 'test content 2', NULL, 'synced');
	`
	if _, err := dbWrapper.Exec(ctx, insertQuery); err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewRAGSyncService(dbWrapper)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "test-1" {
		t.Errorf("expected ID test-1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 3 {
		t.Errorf("expected vector of length 3, got %d", len(records[0].Vector))
	}
}

func TestRagSyncServiceImpl_MarkSynced(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}
	defer sqliteDB.Close()

	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	ctx := context.Background()

	// Setup schema
	setupQuery := `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	if _, err := dbWrapper.Exec(ctx, setupQuery); err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	insertQuery := `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('test-1', 'test content 1', 'pending');
	`
	if _, err := dbWrapper.Exec(ctx, insertQuery); err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewRAGSyncService(dbWrapper)

	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify sync status
	var status string
	var lastSyncAt *time.Time
	err = dbWrapper.Provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = 'test-1'").Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status synced, got %s", status)
	}
	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set, got nil")
	}
}
