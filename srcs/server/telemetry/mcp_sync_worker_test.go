package telemetry

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
    t.Helper()
    sqlDb, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }

    if err := sqlDb.PingContext(context.Background()); err != nil {
        t.Fatalf("failed to ping test sqlite db: %v", err)
    }

    t.Cleanup(func() {
        sqlDb.Close()
    })

    return db.NewSqliteProvider(sqlDb)
}

func TestMcpSyncWorker(t *testing.T) {
    ctx := context.Background()
    provider := newTestProvider(t)

    // Create table for test
    _, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS telemetry_buffer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metric_name TEXT NOT NULL,
            value REAL NOT NULL,
            labels_json TEXT,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status TEXT DEFAULT 'pending'
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    // Insert pending metric
    _, err = provider.Exec(ctx, `INSERT INTO telemetry_buffer (metric_name, value, labels_json, sync_status) VALUES ('test_metric', 42.0, '{}', 'pending')`)
    if err != nil {
        t.Fatalf("failed to insert metric: %v", err)
    }

    worker := NewMcpSyncWorker(provider, "http://localhost/mcp")
    worker.sync(ctx)

    // Verify it was marked synced
    row := provider.QueryRow(ctx, `SELECT sync_status FROM telemetry_buffer WHERE metric_name = 'test_metric' LIMIT 1`)
    var status string
    if err := row.Scan(&status); err != nil {
        t.Fatalf("failed to query status: %v", err)
    }

    if status != "synced" {
        t.Errorf("expected status 'synced', got %q", status)
    }
}
