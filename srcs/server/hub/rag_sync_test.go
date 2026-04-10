package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// Create table
	_, err = d.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(d)
}

func TestRAGSyncService_Flow(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	svc := NewSQLRAGSyncService(provider)

	// Seed some pending records
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "mem1", "pending")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" || records[0].Context != "mem1" {
		t.Fatalf("unexpected record content: %+v", records[0])
	}

	// Mark Synced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify it's synced
	row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = ?", "1")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to scan: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected synced, got %s", status)
	}

	// Process incoming sync (UPSERT)
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "2", Context: "mem2"},
		{ID: "1", Context: "mem1_updated"},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify updates
	row = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = ?", "1")
	var content string
	if err := row.Scan(&content); err != nil {
		t.Fatalf("failed to scan: %v", err)
	}
	if content != "mem1_updated" {
		t.Fatalf("expected mem1_updated, got %s", content)
	}
}
