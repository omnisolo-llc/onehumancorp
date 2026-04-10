package hub

import (
    "context"
    "database/sql"
    "testing"

    _ "modernc.org/sqlite"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) *db.DB {
    t.Helper()
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")

    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }

    // Create necessary tables
    _, err = sqliteDB.Exec(`
        CREATE TABLE IF NOT EXISTS swarm_memory (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
        CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
        t.Fatalf("Failed to create tables: %v", err)
    }

    return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
    db := setupTestDB(t)
    service := NewDefaultRAGSyncService(db.Provider)
    ctx := context.Background()

    // Insert test data
    _, err := db.Provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES (?, ?, ?)", "mem1", "val1", "pending")
    if err != nil {
        t.Fatalf("Failed to insert memory: %v", err)
    }
    _, err = db.Provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context) VALUES (?, ?)", "mem1", "ctx1")
    if err != nil {
        t.Fatalf("Failed to insert embedding: %v", err)
    }

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(records) != 1 {
        t.Fatalf("Expected 1 record, got %d", len(records))
    }

    if records[0].ID != "mem1" || records[0].Context != "ctx1" || records[0].SyncStatus != SyncStatusPending {
        t.Errorf("Unexpected record content: %+v", records[0])
    }
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
    db := setupTestDB(t)
    service := NewDefaultRAGSyncService(db.Provider)
    ctx := context.Background()

    // Insert test data
    _, err := db.Provider.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status) VALUES (?, ?, ?)", "mem1", "val1", "pending")
    if err != nil {
        t.Fatalf("Failed to insert memory: %v", err)
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Verify
    rows, err := db.Provider.Query(ctx, "SELECT sync_status FROM swarm_memory WHERE key = ?", "mem1")
    if err != nil {
        t.Fatalf("Failed to query memory: %v", err)
    }
    defer rows.Close()

    if !rows.Next() {
        t.Fatalf("Expected row not found")
    }

    var status string
    if err := rows.Scan(&status); err != nil {
        t.Fatalf("Failed to scan status: %v", err)
    }

    if status != "synced" {
        t.Errorf("Expected status 'synced', got '%s'", status)
    }
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
    db := setupTestDB(t)
    service := NewDefaultRAGSyncService(db.Provider)
    ctx := context.Background()

    records := []RAGSyncRecord{
        {ID: "mem2", Context: "ctx2"},
    }

    // Test ProcessIncomingSync
    err := service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Verify memory
    rows, err := db.Provider.Query(ctx, "SELECT sync_status FROM swarm_memory WHERE key = ?", "mem2")
    if err != nil {
        t.Fatalf("Failed to query memory: %v", err)
    }
    defer rows.Close()

    if !rows.Next() {
        t.Fatalf("Expected row not found")
    }

    var status string
    if err := rows.Scan(&status); err != nil {
        t.Fatalf("Failed to scan status: %v", err)
    }

    if status != "synced" {
        t.Errorf("Expected status 'synced', got '%s'", status)
    }

    // Verify embedding
    // Only verify if embedding table exists
    var exists int
    _ = db.Provider.QueryRow(ctx, "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='swarm_memory_embeddings'").Scan(&exists)
    if exists > 0 {
        rows2, err := db.Provider.Query(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = ?", "mem2")
        if err != nil {
            t.Fatalf("Failed to query embedding: %v", err)
        }
        defer rows2.Close()

        if !rows2.Next() {
            t.Fatalf("Expected row not found in embeddings")
        }
        var contextStr string
        if err := rows2.Scan(&contextStr); err != nil {
            t.Fatalf("Failed to scan context: %v", err)
        }
        if contextStr != "ctx2" {
            t.Errorf("Expected context 'ctx2', got '%s'", contextStr)
        }
    }
}
