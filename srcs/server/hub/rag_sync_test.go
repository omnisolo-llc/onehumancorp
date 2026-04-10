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
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	// Create autodream_memories table
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'test content 1', '[0.1, 0.2]', 'pending'),
		       ('2', 'test content 2', NULL, 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 2 {
		t.Errorf("expected vector length 2, got %d", len(records[0].Vector))
	}
	if records[0].Vector[0] != 0.1 || records[0].Vector[1] != 0.2 {
		t.Errorf("expected vector [0.1, 0.2], got %v", records[0].Vector)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test content', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	row := provider.QueryRow(ctx, `SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'`)
	var syncStatus string
	var lastSyncAt sql.NullTime
	if err := row.Scan(&syncStatus, &lastSyncAt); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if syncStatus != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", syncStatus)
	}
	if !lastSyncAt.Valid {
		t.Error("expected last_sync_at to be set")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Initial data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'old content', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Content:    "new content",
			Vector:     []float32{0.5, 0.5},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "2",
			Content:    "brand new",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify update
	row := provider.QueryRow(ctx, `SELECT content, sync_status, embedding FROM autodream_memories WHERE id = '1'`)
	var content, syncStatus, embedding string
	if err := row.Scan(&content, &syncStatus, &embedding); err != nil {
		t.Fatalf("failed to scan row 1: %v", err)
	}
	if content != "new content" {
		t.Errorf("expected content 'new content', got %s", content)
	}
	if embedding != "[0.500000,0.500000]" {
		t.Errorf("expected embedding '[0.500000,0.500000]', got %s", embedding)
	}

	// Verify insert
	row2 := provider.QueryRow(ctx, `SELECT content FROM autodream_memories WHERE id = '2'`)
	var content2 string
	if err := row2.Scan(&content2); err != nil {
		t.Fatalf("failed to scan row 2: %v", err)
	}
	if content2 != "brand new" {
		t.Errorf("expected content 'brand new', got %s", content2)
	}
}
