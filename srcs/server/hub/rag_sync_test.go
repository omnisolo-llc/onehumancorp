package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqlDB.Close()
	})

	return db.NewSqliteProvider(sqlDB)
}

func TestFetchPendingSyncsRealDB(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)

	// Ensure the consolidated_memory table has our new columns for testing
	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		source_type TEXT NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("unexpected error creating table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('1', 'system', 'test1', 'sync', 'pending'), ('2', 'system', 'test2', 'sync', 'synced'), ('3', 'system', 'test3', 'sync', 'pending')`)
	if err != nil {
		t.Fatalf("unexpected error inserting data: %v", err)
	}

	svc := NewRAGSyncService(provider)
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSyncedRealDB(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)

	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		source_type TEXT NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("unexpected error creating table: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('1', 'system', 'test1', 'sync', 'pending')`)
	if err != nil {
		t.Fatalf("unexpected error inserting data: %v", err)
	}

	svc := NewRAGSyncService(provider)
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	row := provider.QueryRow(ctx, `SELECT sync_status FROM consolidated_memory WHERE id = '1'`)
	err = row.Scan(&status)
	if err != nil {
		t.Fatalf("unexpected error querying data: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status synced, got %s", status)
	}
}

func TestProcessIncomingSyncRealDB(t *testing.T) {
	ctx := context.Background()
	provider := newTestProvider(t)

	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		source_type TEXT NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`)
	if err != nil {
		t.Fatalf("unexpected error creating table: %v", err)
	}

	svc := NewRAGSyncService(provider)
	records := []RAGSyncRecord{
		{ID: "1", Context: "test", SyncStatus: SyncStatusPending},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var content string
	row := provider.QueryRow(ctx, `SELECT content FROM consolidated_memory WHERE id = '1'`)
	err = row.Scan(&content)
	if err != nil {
		t.Fatalf("unexpected error querying data: %v", err)
	}

	if content != "test" {
		t.Errorf("expected context test, got %s", content)
	}
}
