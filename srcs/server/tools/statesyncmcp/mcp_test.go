package statesyncmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
	syncUpResult   map[string]interface{}
	syncUpErr      error
	syncDownResult map[string]interface{}
	syncDownErr    error
	statusResult   map[string]interface{}
	statusErr      error
}

func (m *mockProvider) SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.syncUpResult, m.syncUpErr
}

func (m *mockProvider) SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.syncDownResult, m.syncDownErr
}

func (m *mockProvider) GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.statusResult, m.statusErr
}

func TestListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&mockProvider{}, true)
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	names := map[string]bool{}
	for _, tool := range tools {
		names[tool.Name] = true
	}

	for _, name := range []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"} {
		if !names[name] {
			t.Errorf("missing tool: %s", name)
		}
	}
}

func TestCallTool_NotLocal(t *testing.T) {
	mcp := NewStateSyncMCP(&mockProvider{}, false)
	res, err := mcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["status"] != "skipped" {
		t.Errorf("expected status 'skipped', got %v", resMap["status"])
	}
}

func TestCallTool_MissingClaims(t *testing.T) {
	mcp := NewStateSyncMCP(&mockProvider{}, true)
	_, err := mcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error due to missing claims")
	}
}

func TestCallTool_SyncUp(t *testing.T) {
	expectedRes := map[string]interface{}{"status": "success", "count": 5}
	provider := &mockProvider{
		syncUpResult: expectedRes,
	}

	mcp := NewStateSyncMCP(provider, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["count"] != 5 {
		t.Errorf("expected count 5, got %v", resMap["count"])
	}
}

func TestCallTool_SyncDown(t *testing.T) {
	expectedRes := map[string]interface{}{"status": "success"}
	provider := &mockProvider{
		syncDownResult: expectedRes,
	}

	mcp := NewStateSyncMCP(provider, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}
}

func TestCallTool_GetStatus(t *testing.T) {
	expectedRes := map[string]interface{}{"status": "success", "pending": 2}
	provider := &mockProvider{
		statusResult: expectedRes,
	}

	mcp := NewStateSyncMCP(provider, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["pending"] != 2 {
		t.Errorf("expected pending 2, got %v", resMap["pending"])
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	mcp := NewStateSyncMCP(&mockProvider{}, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}
}

func TestDBStateSyncProvider_SyncUp_NotSQLite(t *testing.T) {
	// Create mock Postgres DB
	dbWrapper := &db.DB{Provider: &mockDBProvider{isSQLite: false}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")
	_, err := provider.SyncUp(context.Background(), nil)
	if err == nil {
		t.Fatalf("expected error for non-SQLite provider")
	}
}

type mockDBProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}
