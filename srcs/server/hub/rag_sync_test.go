package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	d := &db.DB{Provider: db.NewTestProvider(t)}
	s := NewRAGSyncService(d.Provider)

	ctx := context.Background()

    // create the table because it's in memory db
    _, err := d.Provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS swarm_memory (
            key        TEXT PRIMARY KEY,
            value      TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	_, err = d.Provider.Exec(ctx, "DELETE FROM swarm_memory")
	if err != nil {
		t.Fatalf("Failed to clear table: %v", err)
	}

	_, err = d.Provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('test-key-1', 'test-value-1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert record: %v", err)
	}

	records, err := s.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-key-1" {
		t.Errorf("Expected ID test-key-1, got %s", records[0].ID)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("Expected status pending, got %s", records[0].SyncStatus)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	d := &db.DB{Provider: db.NewTestProvider(t)}
	s := NewRAGSyncService(d.Provider)

	ctx := context.Background()
    _, err := d.Provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS swarm_memory (
            key        TEXT PRIMARY KEY,
            value      TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	_, err = d.Provider.Exec(ctx, "DELETE FROM swarm_memory")
	if err != nil {
		t.Fatalf("Failed to clear table: %v", err)
	}

	_, err = d.Provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES ('test-key-1', 'test-value-1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert record: %v", err)
	}

	err = s.MarkSynced(ctx, []string{"test-key-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	row := d.Provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory WHERE key = 'test-key-1'")
	var status string
	err = row.Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status synced, got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	d := &db.DB{Provider: db.NewTestProvider(t)}
	s := NewRAGSyncService(d.Provider)

	ctx := context.Background()
    _, err := d.Provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS swarm_memory (
            key        TEXT PRIMARY KEY,
            value      TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
        CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	_, err = d.Provider.Exec(ctx, "DELETE FROM swarm_memory")
	if err != nil {
		t.Fatalf("Failed to clear table: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "test-key-1",
			Context:    "test-context-1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = s.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	row := d.Provider.QueryRow(ctx, "SELECT value, sync_status FROM swarm_memory WHERE key = 'test-key-1'")
	var value, status string
	err = row.Scan(&value, &status)
	if err != nil {
		t.Fatalf("Failed to query inserted record: %v", err)
	}

	if value != "test-context-1" {
		t.Errorf("Expected context test-context-1, got %s", value)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status synced, got %s", status)
	}
}
