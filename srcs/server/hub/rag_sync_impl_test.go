package hub

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"database/sql"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}

	// Create table schema
	_, err = sqliteDB.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory (
			key TEXT PRIMARY KEY,
			value TEXT NOT NULL,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	os.Setenv("DATABASE_URL", "sqlite://:memory:")
	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)

	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('2', 'ctx2', 'synced')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got %s", records[0].ID)
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)

	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify status
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory WHERE key = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to verify status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected 'synced', got '%s'", status)
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)

	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "3", Context: "ctx3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var ctxVal string
	err = provider.QueryRow(ctx, "SELECT value FROM swarm_memory WHERE key = '3'").Scan(&ctxVal)
	if err != nil {
		t.Fatalf("failed to verify insert: %v", err)
	}
	if ctxVal != "ctx3" {
		t.Errorf("expected 'ctx3', got '%s'", ctxVal)
	}
}
