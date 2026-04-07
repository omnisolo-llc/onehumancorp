package statesyncmcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// mockProvider implements db.Provider for testing.
type mockProvider struct {
	isSQLite bool
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}

func (m *mockProvider) Close() {}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func TestStateSyncMCP_ListTools(t *testing.T) {
	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, nil)
	m := NewStateSyncMCP(p)

	tools := m.ListTools()
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

func TestStateSyncMCP_CallTool_CloudNop(t *testing.T) {
	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: false}, nil)
	m := NewStateSyncMCP(p)

	ctx := context.Background()
	// No claims needed for no-op

	testCases := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}

	for _, tc := range testCases {
		res, err := m.CallTool(ctx, tc, nil)
		if err != nil {
			t.Fatalf("unexpected error for %s: %v", tc, err)
		}
		resMap, ok := res.(map[string]interface{})
		if !ok {
			t.Fatalf("expected map response for %s", tc)
		}
		if resMap["status"] != "no-op" {
			t.Errorf("expected no-op status for %s, got %v", tc, resMap["status"])
		}
	}
}

func TestStateSyncMCP_CallTool_Unauthorized(t *testing.T) {
	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, nil)
	m := NewStateSyncMCP(p)

	ctx := context.Background()

	testCases := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}

	for _, tc := range testCases {
		_, err := m.CallTool(ctx, tc, nil)
		if err == nil {
			t.Errorf("expected error for %s due to missing claims", tc)
		}
	}
}

func TestStateSyncMCP_CallTool_SQLite(t *testing.T) {
	// Mock HTTP server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/up" && r.Method == "POST" {
			w.WriteHeader(http.StatusOK)
			return
		}
		if r.URL.Path == "/api/v1/sync/down" && r.Method == "GET" {
			w.WriteHeader(http.StatusOK)
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}))
	defer server.Close()

	t.Setenv("OHC_CORE_URL", server.URL)

	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, server.Client())
	m := NewStateSyncMCP(p)

	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test sync_local_to_cloud
	res, err := m.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error for sync_local_to_cloud: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", resMap["status"])
	}

	// Test sync_cloud_to_local
	res, err = m.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error for sync_cloud_to_local: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", resMap["status"])
	}

	// Test get_sync_status
	res, err = m.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error for get_sync_status: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "ok" {
		t.Errorf("expected ok status, got %v", resMap["status"])
	}
}

func TestStateSyncMCP_CallTool_Unknown(t *testing.T) {
	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, nil)
	m := NewStateSyncMCP(p)

	_, err := m.CallTool(context.Background(), "unknown_tool", nil)
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}
}

func TestStateSyncMCP_HTTPError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("internal error"))
	}))
	defer server.Close()

	t.Setenv("OHC_CORE_URL", server.URL)

	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, server.Client())
	m := NewStateSyncMCP(p)

	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := m.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Errorf("expected error for sync_local_to_cloud due to 500 response")
	}

	_, err = m.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Errorf("expected error for sync_cloud_to_local due to 500 response")
	}
}

func TestStateSyncMCP_EnvFallback(t *testing.T) {
	// Unset OHC_CORE_URL to test fallback to localhost:8080
	t.Setenv("OHC_CORE_URL", "")

	p := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true}, nil)
	m := NewStateSyncMCP(p)

	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Will fail because localhost:8080 is not running, but that's expected
	_, err := m.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Errorf("expected error for sync_local_to_cloud due to connection refused")
	}

	_, err = m.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Errorf("expected error for sync_cloud_to_local due to connection refused")
	}
}
