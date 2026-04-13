package telemetry

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestMcpSyncWorker(t *testing.T) {
    sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

    ctx := context.Background()
    _, err = provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS telemetry_buffer (
            id TEXT PRIMARY KEY,
            metric_name TEXT NOT NULL,
            value REAL NOT NULL,
            labels_json TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            sync_status TEXT DEFAULT 'PENDING'
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    _, err = provider.Exec(ctx, `
        INSERT INTO telemetry_buffer (id, metric_name, value, labels_json)
        VALUES ('1', 'test_metric', 10.0, '{}')
    `)
    if err != nil {
        t.Fatalf("failed to insert mock metric: %v", err)
    }

    worker := NewMcpSyncWorker(provider)
    worker.syncMetrics(ctx)

    var status string
    err = provider.QueryRow(ctx, "SELECT sync_status FROM telemetry_buffer WHERE id = '1'").Scan(&status)
    if err != nil {
        t.Fatalf("failed to query status: %v", err)
    }

    if status != "SYNCED" {
        t.Errorf("expected SYNCED, got %s", status)
    }
}
