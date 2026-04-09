package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestDefaultRAGSyncService(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

    // Explicitly create the table for testing
    _, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding TEXT,
            source_mission_id TEXT,
            sync_status TEXT DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
        t.Fatalf("failed to create autodream_memories table: %v", err)
    }

	service := NewDefaultRAGSyncService(provider)

	// Set up initial data
	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-id-1', 'test content 1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-id-2', 'test content 2', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-id-1" {
		t.Errorf("expected ID test-id-1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after mark synced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{
			ID:      "test-id-3",
			Context: "test content 3",
		},
		{
			ID:      "test-id-1", // Update existing
			Context: "updated test content 1",
		},
	}

	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	row := provider.QueryRow(ctx, `SELECT content, sync_status FROM autodream_memories WHERE id = 'test-id-3'`)
	var content, syncStatus string
	if err := row.Scan(&content, &syncStatus); err != nil {
		t.Fatalf("failed to query test-id-3: %v", err)
	}
	if content != "test content 3" || syncStatus != "synced" {
		t.Errorf("unexpected values for test-id-3: content=%s, syncStatus=%s", content, syncStatus)
	}

	row = provider.QueryRow(ctx, `SELECT content, sync_status FROM autodream_memories WHERE id = 'test-id-1'`)
	if err := row.Scan(&content, &syncStatus); err != nil {
		t.Fatalf("failed to query test-id-1: %v", err)
	}
	if content != "updated test content 1" || syncStatus != "synced" {
		t.Errorf("unexpected values for test-id-1: content=%s, syncStatus=%s", content, syncStatus)
	}
}
