package hub

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncEngine_StartStop(t *testing.T) {
	os.Setenv("DATABASE_URL", "sqlite://test_sync.db")
	defer os.Remove("test_sync.db")

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer dbWrapper.Close()

	// Ensure the table exists for testing using standard SQL
	_, err = dbWrapper.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	engine := NewRAGSyncEngine(dbWrapper, 100*time.Millisecond, "")

	engine.Start(ctx)
	time.Sleep(200 * time.Millisecond)
	engine.Stop()
}

func TestRAGSyncEngine_FetchMark(t *testing.T) {
	os.Setenv("DATABASE_URL", "sqlite://test_sync2.db")
	defer os.Remove("test_sync2.db")

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer dbWrapper.Close()

	// Ensure the table exists for testing using standard SQL
	_, err = dbWrapper.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	// Insert test data
	_, err = dbWrapper.Exec(ctx, "INSERT INTO autodream_memories (content, sync_status) VALUES ($1, $2)", "test memory", SyncStatusPending)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	engine := NewRAGSyncEngine(dbWrapper, 100*time.Millisecond, "")

	records, err := engine.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].Context != "test memory" {
		t.Fatalf("expected context 'test memory', got '%s'", records[0].Context)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Fatalf("expected status 'pending', got '%s'", records[0].SyncStatus)
	}

	err = engine.MarkSynced(ctx, []string{records[0].ID})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify status updated
	records2, err := engine.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records2) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records2))
	}
}
