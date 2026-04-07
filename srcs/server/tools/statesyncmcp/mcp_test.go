package statesyncmcp

import (
	"bytes"
	"context"
	"database/sql"
	"io"
	"net/http"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// newTestProvider creates a new in-memory SQLite database provider for testing.
func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqlDB.Close()
	})

	return db.NewSqliteProvider(sqlDB)
}

// mockRoundTripper intercepts HTTP requests.
type mockRoundTripper struct {
	roundTripFunc func(req *http.Request) (*http.Response, error)
}

func (m *mockRoundTripper) RoundTrip(req *http.Request) (*http.Response, error) {
	return m.roundTripFunc(req)
}

func TestDBStateSyncMCP_ListTools(t *testing.T) {
	mockDB := newTestProvider(t)
	mcp := NewDBStateSyncMCP(mockDB, nil)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	expectedTools := map[string]bool{
		"sync_local_to_cloud": true,
		"sync_cloud_to_local": true,
		"get_sync_status":     true,
	}

	for _, tool := range tools {
		if !expectedTools[tool.Name] {
			t.Errorf("unexpected tool: %s", tool.Name)
		}
	}
}

func TestDBStateSyncMCP_CallTool_MissingClaims(t *testing.T) {
	mockDB := newTestProvider(t)
	mcp := NewDBStateSyncMCP(mockDB, nil)

	_, err := mcp.CallTool(context.Background(), "get_sync_status", nil)
	if err == nil {
		t.Fatal("expected error for missing claims, got nil")
	}
	if err.Error() != "unauthorized: missing claims" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestDBStateSyncMCP_GetStatus(t *testing.T) {
	mockDB := newTestProvider(t) // Default is SQLite
	mcp := NewDBStateSyncMCP(mockDB, nil)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	statusMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map[string]interface{}")
	}

	if statusMap["mode"] != "standalone" {
		t.Errorf("expected mode standalone, got %v", statusMap["mode"])
	}
}

func TestDBStateSyncMCP_SyncUp_SQLite(t *testing.T) {
	mockDB := newTestProvider(t)
	ctx := context.Background()
	_, _ = mockDB.Exec(ctx, "CREATE TABLE kairos_tasks (id TEXT PRIMARY KEY, status TEXT, payload TEXT, updated_at TEXT)")
	_, _ = mockDB.Exec(ctx, "INSERT INTO kairos_tasks (id, status, payload, updated_at) VALUES ('1', 'pending', 'data', '2023-01-01T00:00:00Z')")

	client := &http.Client{
		Transport: &mockRoundTripper{
			roundTripFunc: func(req *http.Request) (*http.Response, error) {
				if req.Method != "POST" {
					t.Errorf("expected POST, got %s", req.Method)
				}
				return &http.Response{
					StatusCode: http.StatusOK,
					Body:       io.NopCloser(bytes.NewBufferString(`{"status":"ok"}`)),
				}, nil
			},
		},
	}

	mcp := NewDBStateSyncMCP(mockDB, client)

	authCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	res, err := mcp.CallTool(authCtx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status")
	}
}

func TestDBStateSyncMCP_SyncDown_SQLite(t *testing.T) {
	mockDB := newTestProvider(t)
	ctx := context.Background()
	_, _ = mockDB.Exec(ctx, "CREATE TABLE kairos_tasks (id TEXT PRIMARY KEY, status TEXT, payload TEXT, updated_at TEXT)")

	client := &http.Client{
		Transport: &mockRoundTripper{
			roundTripFunc: func(req *http.Request) (*http.Response, error) {
				if req.Method != "GET" {
					t.Errorf("expected GET, got %s", req.Method)
				}
				body := `{"tasks": [{"id": "2", "status": "completed", "payload": "result", "updated_at": "2023-01-02T00:00:00Z"}]}`
				return &http.Response{
					StatusCode: http.StatusOK,
					Body:       io.NopCloser(bytes.NewBufferString(body)),
				}, nil
			},
		},
	}

	mcp := NewDBStateSyncMCP(mockDB, client)

	authCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	res, err := mcp.CallTool(authCtx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status")
	}

	// Verify the row was inserted
	row := mockDB.QueryRow(ctx, "SELECT status FROM kairos_tasks WHERE id = '2'")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}
	if status != "completed" {
		t.Errorf("expected status completed, got %s", status)
	}
}

// Add mock PostgreSQL provider test to ensure it acts as a no-op
type mockPGProvider struct {
    db.Provider
}

func (m *mockPGProvider) IsSQLite() bool { return false }

func TestDBStateSyncMCP_PG_NoOp(t *testing.T) {
	pgDB := &mockPGProvider{}
	mcp := NewDBStateSyncMCP(pgDB, nil)

	authCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	res, err := mcp.CallTool(authCtx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
    resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status")
	}

    resDown, err := mcp.CallTool(authCtx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
    resDownMap := resDown.(map[string]interface{})
	if resDownMap["status"] != "success" {
		t.Errorf("expected success status")
	}
}

func TestDBStateSyncMCP_UnknownTool(t *testing.T) {
	mockDB := newTestProvider(t)
	mcp := NewDBStateSyncMCP(mockDB, nil)

	authCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	_, err := mcp.CallTool(authCtx, "unknown_tool", nil)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
    if err.Error() != "unknown tool: unknown_tool" {
        t.Errorf("unexpected error: %v", err)
    }
}
