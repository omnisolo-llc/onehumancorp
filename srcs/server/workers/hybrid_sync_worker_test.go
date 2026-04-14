package workers

import (
    "context"
    "database/sql"
    "net/http"
    "net/http/httptest"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestHybridSyncWorker(t *testing.T) {
    sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite in-memory db: %v", err)
    }

    // Create the schema
    _, err = sqlDB.Exec(`CREATE TABLE swarm_memory_embeddings (
        memory_id TEXT PRIMARY KEY,
        context TEXT NOT NULL,
        sync_enabled BOOLEAN DEFAULT FALSE,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at TIMESTAMPTZ NULL
    )`)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    // Insert dummy data
    _, err = sqlDB.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, sync_enabled) VALUES ('123', '{"data":"test"}', true)`)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    p := db.NewSqliteProvider(sqlDB)

    // Create a mock server
    mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        if r.URL.Path != "/api/sync/vectors" {
            t.Errorf("expected path /api/sync/vectors, got %s", r.URL.Path)
        }
        w.WriteHeader(http.StatusOK)
    }))
    defer mockServer.Close()

    w := NewHybridSyncWorker(p, mockServer.URL, nil)

    // Run poll
    w.poll(context.Background())

    // Verify it updated the DB
    var syncStatus string
    err = sqlDB.QueryRow(`SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '123'`).Scan(&syncStatus)
    if err != nil {
        t.Fatalf("failed to query updated data: %v", err)
    }

    if syncStatus != "synced" {
        t.Errorf("expected sync_status to be 'synced', got '%s'", syncStatus)
    }
}
