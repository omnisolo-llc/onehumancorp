package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	// Setup in-memory SQLite DB for testing
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	// Create tables required for testing
	createTableQuery := `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	_, err = sqlDB.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	insertQuery := `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test context', 'pending')
	`
	_, err = provider.Exec(ctx, insertQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("Expected ID 1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced worked by fetching again
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records after sync, got %d", len(records))
	}

	// Test ProcessIncomingSync (insert new)
	incoming := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "new context from cloud",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insert
	var content string
	var status string
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '2'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("Failed to query inserted record: %v", err)
	}
	if content != "new context from cloud" {
		t.Errorf("Expected 'new context from cloud', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}

	// Test ProcessIncomingSync (update existing)
	incomingUpdate := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "updated context from cloud",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incomingUpdate)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify update
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '1'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}
	if content != "updated context from cloud" {
		t.Errorf("Expected 'updated context from cloud', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}
