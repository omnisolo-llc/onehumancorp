package telemetry

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/google/uuid"
	_ "modernc.org/sqlite"
)

type mockProvider struct {
	db *sql.DB
}

func (m *mockProvider) DB() *sql.DB {
	return m.db
}

func TestMcpSyncWorker_SyncOnce(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()
	provider := &mockProvider{db: db}

	// Create table if not exists (sqlite memory db might need it for testing)
	_, err = provider.DB().Exec(`
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			labels_json TEXT,
			timestamp DATETIME NOT NULL,
			sync_status TEXT DEFAULT 'pending'
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	// Insert test data
	id1 := uuid.New().String()
	_, err = provider.DB().Exec(`
		INSERT INTO telemetry_buffer (id, metric_name, value, labels_json, timestamp, sync_status)
		VALUES (?, ?, ?, ?, ?, 'pending')
	`, id1, "test_metric_1", 42.0, "{}", time.Now())
	if err != nil {
		t.Fatalf("Failed to insert test metric: %v", err)
	}

	worker := NewMcpSyncWorker(provider, 100*time.Millisecond)

	ctx := context.Background()
	worker.syncOnce(ctx)

	// Verify sync status
	var status string
	err = provider.DB().QueryRow("SELECT sync_status FROM telemetry_buffer WHERE id = ?", id1).Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}
