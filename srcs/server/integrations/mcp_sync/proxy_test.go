package mcp_sync_test

import (
	"context"
	"testing"
    "path/filepath"
    "database/sql"
    _ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/integrations/mcp_sync"
)

func TestEnqueueSync(t *testing.T) {
	ctx := context.Background()

    // Using memory sharing because of table creates
    dbPath := filepath.Join(t.TempDir(), "test.db")
    sqlDB, err := sql.Open("sqlite", dbPath)
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)
    defer provider.Close()

    // Create table for test
    _, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS hybrid_mcp_sync_queue (
    id VARCHAR(255) PRIMARY KEY,
    tool_name VARCHAR(255) NOT NULL,
    arguments TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );`)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

	proxy := mcp_sync.NewMcpSyncProxy(provider)

	err = proxy.EnqueueSync(ctx, "test-id-1", "jira_create", map[string]interface{}{"title": "Fix bug"})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM hybrid_mcp_sync_queue WHERE id = 'test-id-1'")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record, got %d", count)
	}
}
