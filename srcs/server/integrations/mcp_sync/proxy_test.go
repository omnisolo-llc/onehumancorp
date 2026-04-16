package mcp_sync

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestMcpSyncProxy(t *testing.T) {
	provider := db.NewTestProvider(t)

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS hybrid_mcp_sync_queue (
			id VARCHAR(36) PRIMARY KEY,
			tool_name VARCHAR(255) NOT NULL,
			payload TEXT NOT NULL,
			status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			synced_at TIMESTAMP,
			error_message TEXT
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup test db: %v", err)
	}

	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer mockServer.Close()

	proxy := NewMcpSyncProxy(provider, mockServer.Client())

	id, err := proxy.BufferToolExecution(ctx, "test_tool", "{\"args\": {}}")
	if err != nil {
		t.Fatalf("failed to buffer: %v", err)
	}
	if id == "" {
		t.Fatalf("expected non-empty id")
	}

	err = proxy.SyncToCloud(ctx, mockServer.URL, "spiffe://local.ohc/daemon")
	if err != nil {
		t.Fatalf("failed to sync: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT status FROM hybrid_mcp_sync_queue WHERE id = $1", id)
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to scan status: %v", err)
	}
	if status != "SYNCED" {
		t.Fatalf("expected SYNCED, got %s", status)
	}
}
