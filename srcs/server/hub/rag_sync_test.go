package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	// Setup in-memory SQLite DB
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite DB: %v", err)
	}
	defer sqlDB.Close()

	// Initialize schema
	schema := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	if _, err := sqlDB.Exec(schema); err != nil {
		t.Fatalf("Failed to initialize schema: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	service := NewRAGSyncServiceImpl(provider)
	ctx := context.Background()

	// Insert test data
	now := time.Now().Truncate(time.Second) // Truncate to second for SQLite compat
	insertQuery := `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('test-id-1', 'memory 1', 'pending'),
		       ('test-id-2', 'memory 2', 'pending')
	`
	if _, err := sqlDB.Exec(insertQuery); err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("Expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	var status string
	var lastSyncAt sql.NullTime
	err = sqlDB.QueryRow("SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = 'test-id-1'").Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status to be 'synced', got '%s'", status)
	}
	if !lastSyncAt.Valid {
		t.Errorf("Expected last_sync_at to be valid")
	}

	// Check that the other one is still pending
	err = sqlDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = 'test-id-2'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query record 2: %v", err)
	}
	if status != "pending" {
		t.Errorf("Expected status to be 'pending', got '%s'", status)
	}

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{
			ID:         "test-id-3",
			Context:    "memory 3",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
		{
			ID:         "test-id-1", // Will trigger conflict resolution (ON CONFLICT)
			Context:    "memory 1 updated",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify test-id-3 was inserted
	var content string
	err = sqlDB.QueryRow("SELECT content, sync_status FROM autodream_memories WHERE id = 'test-id-3'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("Failed to query inserted record test-id-3: %v", err)
	}
	if content != "memory 3" {
		t.Errorf("Expected content 'memory 3', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}

	// Verify test-id-1 was updated
	err = sqlDB.QueryRow("SELECT content FROM autodream_memories WHERE id = 'test-id-1'").Scan(&content)
	if err != nil {
		t.Fatalf("Failed to query updated record test-id-1: %v", err)
	}
	if content != "memory 1 updated" {
		t.Errorf("Expected content 'memory 1 updated', got '%s'", content)
	}
}
