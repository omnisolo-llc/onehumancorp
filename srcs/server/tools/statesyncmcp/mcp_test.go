package statesyncmcp

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	// Setup mock tables
	_, err = sqliteDB.Exec(`
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL,
			agent_id TEXT,
			priority TEXT NOT NULL,
			payload TEXT,
			locked_until TIMESTAMP,
			created_at TIMESTAMP,
			updated_at TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create mock table: %v", err)
	}
	return db.NewSqliteProvider(sqliteDB)
}

// Mock PG provider to test cloud mode behavior
type mockPGProvider struct {
	db.Provider
}

func (m *mockPGProvider) IsSQLite() bool {
	return false
}

func TestStateSyncMCP_ListTools(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	mcp := NewStateSyncMCP(provider)
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestStateSyncMCP_CallTool_MissingClaims(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	mcp := NewStateSyncMCP(provider)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("Expected error for missing claims, got nil")
	}
	if err.Error() != "unauthorized: missing claims" {
		t.Fatalf("Unexpected error message: %v", err)
	}
}

func TestStateSyncMCP_CallTool_CloudModeFallback(t *testing.T) {
	pgProvider := &mockPGProvider{}
	mcp := NewStateSyncMCP(pgProvider)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
		Roles:          []string{"user"},
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("Expected no error for cloud fallback, got %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map result, got %T", res)
	}

	if resMap["status"] != "noop" {
		t.Fatalf("Expected status noop, got %v", resMap["status"])
	}
}

func TestStateSyncMCP_CallTool_SQLiteSyncUp(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	// Insert test data
	_, err := provider.Exec(context.Background(), `
		INSERT INTO shared_tasks (id, organization_id, title, status, priority, created_at, updated_at)
		VALUES ('task-1', 'tenant-1', 'Test Task', 'PENDING', 'P0', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	// Create mock server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	mcp := NewStateSyncMCP(provider)
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map result, got %T", res)
	}

	if resMap["status"] != "success" || resMap["synced_items"] != 1 {
		t.Fatalf("Unexpected result: %v", resMap)
	}
}

func TestStateSyncMCP_CallTool_SQLiteSyncDown(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	// Create mock server returning JSON tasks
	tasks := []SyncTask{
		{
			ID:             "task-remote-1",
			OrganizationID: "tenant-1",
			Title:          "Remote Task",
			Status:         "PENDING",
			Priority:       "P1",
			CreatedAt:      time.Now(),
			UpdatedAt:      time.Now(),
		},
	}
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		json.NewEncoder(w).Encode(tasks)
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	mcp := NewStateSyncMCP(provider)
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map result, got %T", res)
	}

	if resMap["status"] != "success" || resMap["fetched_items"] != 1 {
		t.Fatalf("Unexpected result: %v", resMap)
	}

	// Verify task was inserted
	row := provider.QueryRow(context.Background(), "SELECT title FROM shared_tasks WHERE id = 'task-remote-1'")
	var title string
	if err := row.Scan(&title); err != nil {
		t.Fatalf("Failed to query inserted task: %v", err)
	}
	if title != "Remote Task" {
		t.Fatalf("Expected title 'Remote Task', got '%s'", title)
	}
}

func TestStateSyncMCP_CallTool_SQLiteGetStatus(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	// Insert test data
	_, err := provider.Exec(context.Background(), `
		INSERT INTO shared_tasks (id, organization_id, title, status, priority, created_at, updated_at)
		VALUES ('task-1', 'tenant-1', 'Test Task', 'PENDING', 'P0', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	mcp := NewStateSyncMCP(provider)
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map result, got %T", res)
	}

	if resMap["status"] != "success" || resMap["pending_up"] != 1 {
		t.Fatalf("Unexpected status: %v", resMap)
	}
}
