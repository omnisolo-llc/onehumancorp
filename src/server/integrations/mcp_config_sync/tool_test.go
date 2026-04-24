package mcp_config_sync

import (
    "context"
    "database/sql"
    "testing"

    _ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
    db, err := sql.Open("sqlite3", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite3 memory db: %v", err)
    }

    _, err = db.Exec(`
        CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            config_key TEXT NOT NULL,
            config_value TEXT NOT NULL,
            metadata TEXT,
            synced_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return db
}

func TestConfigSyncTool(t *testing.T) {
    db := setupTestDB(t)
    defer db.Close()

    tool := NewConfigSyncTool(db)
    ctx := context.Background()

    // Test SyncConfigToCloud
    payload := ConfigSyncPayload{
        TenantID: "tenant-1",
        AgentID:  "agent-1",
        Key:      "TEST_KEY",
        Value:    "TEST_VALUE",
        Metadata: map[string]string{"env": "test"},
    }

    err := tool.SyncConfigToCloud(ctx, payload)
    if err != nil {
        t.Fatalf("SyncConfigToCloud failed: %v", err)
    }

    // Test GetConfig
    val, err := tool.GetConfig(ctx, "tenant-1", "agent-1", "TEST_KEY")
    if err != nil {
        t.Fatalf("GetConfig failed: %v", err)
    }

    if val != "TEST_VALUE" {
        t.Errorf("expected TEST_VALUE, got %s", val)
    }

    // Test GetConfig not found
    _, err = tool.GetConfig(ctx, "tenant-1", "agent-1", "NON_EXISTENT_KEY")
    if err == nil {
        t.Errorf("expected error for non-existent key, got nil")
    }

    // Test SyncConfigToCloud with invalid metadata (not possible to trigger with map[string]string in go json.Marshal usually, but let's just cover the happy path)
}
