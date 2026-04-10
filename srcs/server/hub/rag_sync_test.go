package hub

import (
	"context"
	"testing"
	"time"
    "database/sql"
    _ "modernc.org/sqlite"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService_Flow(t *testing.T) {
	ctx := context.Background()

    // Setup an in-memory SQLite database for testing the provider
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open memory sqlite db: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)
    defer provider.Close()

    // Create the autodream_memories table
    _, err = provider.Exec(ctx, `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMPTZ NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    // Insert some pending records
    _, err = provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status)
        VALUES ('1', 'test context 1', 'pending'), ('2', 'test context 2', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert pending records: %v", err)
    }

    service := NewRAGSyncService(provider)

	// 1. Fetch pending syncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// 2. Mark synced
	var idsToSync []string
	for _, p := range pending {
		idsToSync = append(idsToSync, p.ID)
	}
	err = service.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

    // Verify marked synced
    pendingAfterMark, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfterMark) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfterMark))
	}

	// 3. Process incoming sync (e.g. cloud side)
    incomingRecords := []RAGSyncRecord{
        {
            ID: "3",
            Context: "incoming context 3",
            SyncStatus: SyncStatusSynced,
            LastSyncAt: time.Now(),
        },
    }
	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

    // Check if the record was inserted
    row := provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '3'")
    var content string
    if err := row.Scan(&content); err != nil {
        t.Fatalf("failed to query inserted record: %v", err)
    }
    if content != "incoming context 3" {
        t.Fatalf("expected content 'incoming context 3', got '%s'", content)
    }
}
