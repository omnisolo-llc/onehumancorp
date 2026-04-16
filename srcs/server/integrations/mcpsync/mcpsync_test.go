package mcpsync

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestMcpSyncProxy(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS hybrid_mcp_sync_queue (
			id VARCHAR(255) PRIMARY KEY,
			tool_name VARCHAR(255) NOT NULL,
			arguments TEXT NOT NULL,
			status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	proxy := NewMcpSyncProxy(provider)

	err = proxy.BufferIntegrationMetadata(ctx, "sync-1", "test_tool", "{}")
	if err != nil {
		t.Fatalf("expected no error buffering metadata, got %v", err)
	}

	err = proxy.SyncToCloudGateway(ctx)
	if err != nil {
		t.Fatalf("expected no error syncing, got %v", err)
	}

	rows, err := provider.Query(ctx, `SELECT status FROM hybrid_mcp_sync_queue WHERE id = 'sync-1'`)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected 1 row, got 0")
	}

	var status string
	err = rows.Scan(&status)
	if err != nil {
		t.Fatalf("failed to scan status: %v", err)
	}

	if status != "SYNCED" {
		t.Errorf("expected status 'SYNCED', got %s", status)
	}
}
