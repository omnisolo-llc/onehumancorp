package mcp_sync

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE hybrid_mcp_sync_queue (
			id VARCHAR(255) PRIMARY KEY,
			tool_name VARCHAR(255) NOT NULL,
			arguments TEXT NOT NULL,
			status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return provider
}

func TestMcpSyncProxy_BufferIntegrationMetadata(t *testing.T) {
	provider := setupTestDB(t)
	proxy := NewMcpSyncProxy(provider, "http://dummy")

	ctx := context.Background()
	args := map[string]string{"key": "value"}
	err := proxy.BufferIntegrationMetadata(ctx, "test_tool", args)
	if err != nil {
		t.Fatalf("Failed to buffer metadata: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM hybrid_mcp_sync_queue").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 item in queue, got %d", count)
	}
}

func TestMcpSyncProxy_SyncToCloudGateway(t *testing.T) {
	provider := setupTestDB(t)

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-Spiffe-Token") != "test-token" {
			t.Errorf("Expected SPIFFE token in header")
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	os.Setenv("SPIFFE_IDENTITY_TOKEN", "test-token")
	defer os.Unsetenv("SPIFFE_IDENTITY_TOKEN")

	proxy := NewMcpSyncProxy(provider, server.URL)

	ctx := context.Background()
	proxy.BufferIntegrationMetadata(ctx, "test_tool", map[string]string{"key": "value"})

	count, err := proxy.SyncToCloudGateway(ctx)
	if err != nil {
		t.Fatalf("SyncToCloudGateway failed: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 synced item, got %d", count)
	}

	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM hybrid_mcp_sync_queue LIMIT 1").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "SYNCED" {
		t.Errorf("Expected status SYNCED, got %s", status)
	}
}

func TestMcpSyncProxy_SyncToCloudGateway_EmptyQueue(t *testing.T) {
	provider := setupTestDB(t)
	proxy := NewMcpSyncProxy(provider, "http://dummy")

	ctx := context.Background()
	count, err := proxy.SyncToCloudGateway(ctx)
	if err != nil {
		t.Fatalf("SyncToCloudGateway failed: %v", err)
	}
	if count != 0 {
		t.Errorf("Expected 0 synced items, got %d", count)
	}
}

func TestMcpSyncProxy_SyncToCloudGateway_ErrorOnUpdate(t *testing.T) {
	provider := setupTestDB(t)
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(provider, server.URL)
	ctx := context.Background()
	proxy.BufferIntegrationMetadata(ctx, "test_tool", map[string]string{"key": "value"})

	// manually drop the table to simulate an error during update
	// Note: in sqlite dropping the table after starting the transaction may cause locks
	// So we can simulate it differently, but for now since SQLite locks the whole DB this will just throw an error.
	provider.Exec(ctx, "DROP TABLE hybrid_mcp_sync_queue")

	count, err := proxy.SyncToCloudGateway(ctx)
	if err == nil {
		t.Fatalf("Expected error when syncing items due to dropped table")
	}
	if count != 0 {
		t.Errorf("Expected 0 synced items, got %d", count)
	}
}
