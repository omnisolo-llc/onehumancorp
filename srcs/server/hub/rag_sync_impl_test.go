package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRagSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory (
			key        TEXT PRIMARY KEY,
			value      TEXT NOT NULL,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('test-id-1', 'test context', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "test-id-1" || records[0].Context != "test context" || records[0].SyncStatus != SyncStatusPending {
		t.Errorf("record data mismatch: %+v", records[0])
	}
}

func TestRagSyncServiceImpl_MarkSynced(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory (
			key        TEXT PRIMARY KEY,
			value      TEXT NOT NULL,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('test-id-2', 'test context 2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(provider)

	err = service.MarkSynced(ctx, []string{"test-id-2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory WHERE key = 'test-id-2'")
	var status string
	var lastSyncAt *time.Time
	err = row.Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", status)
	}
	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set, got nil")
	}
}
