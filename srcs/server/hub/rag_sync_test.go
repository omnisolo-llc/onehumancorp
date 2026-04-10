package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric/noop"
)

func init() {
	// Provide a noop meter provider for tests to avoid panic if metrics are recorded
	otel.SetMeterProvider(noop.NewMeterProvider())
}

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()

	// Drop and recreate table for clean state
	_, err = provider.Exec(ctx, "DROP TABLE IF EXISTS autodream_memories")
	if err != nil {
		t.Fatalf("failed to drop table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewHybridRAGSyncService(provider)

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'test context 1', '[0.1, 0.2]', 'pending'),
		       ('2', 'test context 2', NULL, 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected record ID '1', got '%s'", records[0].ID)
	}

	if len(records[0].Vector) != 2 {
		t.Errorf("expected vector length 2, got %d", len(records[0].Vector))
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewHybridRAGSyncService(provider)

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test context 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify
	row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'")
	var status string
	err = row.Scan(&status)
	if err != nil {
		t.Fatalf("failed to query row: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewHybridRAGSyncService(provider)

	now := time.Now()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "new context from standalone",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insert
	row := provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '1'")
	var content, status string
	err = row.Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query row: %v", err)
	}

	if content != "new context from standalone" {
		t.Errorf("expected content 'new context from standalone', got '%s'", content)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	// Test Update
	records[0].Context = "updated context"
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync update failed: %v", err)
	}

	row = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '1'")
	err = row.Scan(&content)
	if err != nil {
		t.Fatalf("failed to query row after update: %v", err)
	}

	if content != "updated context" {
		t.Errorf("expected content 'updated context', got '%s'", content)
	}
}
