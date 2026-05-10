package telemetry

import (
	"context"
	"testing"

	"onehumancorp/srcs/server/db"
)

func TestMcpSyncWorker_SyncPendingMetrics(t *testing.T) {
	// Initialize a test database provider (SQLite in-memory)
	// We use a uniquely named URI for memory DB per test to prevent test collisions as specified in guidelines


	// Real test db initialization. The global provider approach in the existing codebase test helper sets up a generic one. Let's create our own unique one if possible, or just use the helper and ensure we manage schema properly.
	provider := db.NewTestProvider(t)
	// Close it later
	defer provider.DB.Close()

	ctx := context.Background()

	// 1. Setup Schema
	_, err := provider.DB.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			labels_json TEXT,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status TEXT DEFAULT 'pending'
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// 2. Insert test data
	_, err = provider.DB.ExecContext(ctx, `
		INSERT INTO telemetry_buffer (id, metric_name, value, sync_status) VALUES
		('m1', 'cpu_usage', 45.5, 'pending'),
		('m2', 'memory_usage', 1024.0, 'pending'),
		('m3', 'disk_io', 20.0, 'synced'); -- Already synced, shouldn't be touched
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// 3. Initialize worker and run sync
	worker := NewMcpSyncWorker(provider)
	if err := worker.SyncPendingMetrics(ctx); err != nil {
		t.Fatalf("SyncPendingMetrics failed: %v", err)
	}

	// 4. Verify results
	rows, err := provider.DB.QueryContext(ctx, "SELECT id, sync_status FROM telemetry_buffer ORDER BY id")
	if err != nil {
		t.Fatalf("failed to query results: %v", err)
	}
	defer rows.Close()

	expectedStatus := map[string]string{
		"m1": "synced",
		"m2": "synced",
		"m3": "synced",
	}

	for rows.Next() {
		var id, status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("failed to scan row: %v", err)
		}

		expected, ok := expectedStatus[id]
		if !ok {
			t.Fatalf("unexpected metric id found: %s", id)
		}
		if status != expected {
			t.Errorf("expected status %s for metric %s, got %s", expected, id, status)
		}
	}

	if err := rows.Err(); err != nil {
		t.Fatalf("error iterating over results: %v", err)
	}
}