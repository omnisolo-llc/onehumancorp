package statesyncmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockStateSyncProvider struct {
	syncUpFunc    func(ctx context.Context, claims *auth.Claims) (interface{}, error)
	syncDownFunc  func(ctx context.Context, claims *auth.Claims) (interface{}, error)
	getStatusFunc func(ctx context.Context, claims *auth.Claims) (interface{}, error)
}

func (m *mockStateSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.syncUpFunc != nil {
		return m.syncUpFunc(ctx, claims)
	}
	return nil, nil
}

func (m *mockStateSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.syncDownFunc != nil {
		return m.syncDownFunc(ctx, claims)
	}
	return nil, nil
}

func (m *mockStateSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.getStatusFunc != nil {
		return m.getStatusFunc(ctx, claims)
	}
	return nil, nil
}

type mockDBProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func TestListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: true}, &mockStateSyncProvider{})
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	expectedNames := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}
	for i, tool := range tools {
		if tool.Name != expectedNames[i] {
			t.Errorf("Expected tool %d to be %s, got %s", i, expectedNames[i], tool.Name)
		}
	}
}

func TestCallTool_MissingClaims(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: true}, &mockStateSyncProvider{})
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Error("Expected error for missing claims, got nil")
	}
	if err.Error() != "unauthorized: missing claims" {
		t.Errorf("Expected 'unauthorized: missing claims', got '%s'", err.Error())
	}
}

func TestCallTool_CloudModeFallback(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: false}, &mockStateSyncProvider{})
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Errorf("Expected map[string]interface{}, got %T", res)
	}
	if resMap["status"] != "success" || resMap["mode"] != "cloud" {
		t.Errorf("Unexpected result map: %v", resMap)
	}
}

func TestCallTool_Tools(t *testing.T) {
	syncUpCalled := false
	syncDownCalled := false
	getStatusCalled := false

	provider := &mockStateSyncProvider{
		syncUpFunc: func(ctx context.Context, claims *auth.Claims) (interface{}, error) {
			syncUpCalled = true
			return "sync_up_res", nil
		},
		syncDownFunc: func(ctx context.Context, claims *auth.Claims) (interface{}, error) {
			syncDownCalled = true
			return "sync_down_res", nil
		},
		getStatusFunc: func(ctx context.Context, claims *auth.Claims) (interface{}, error) {
			getStatusCalled = true
			return "get_status_res", nil
		},
	}

	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: true}, provider)
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Sync up
	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if res != "sync_up_res" || !syncUpCalled {
		t.Errorf("syncUp failed")
	}

	// Sync down
	res, err = mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if res != "sync_down_res" || !syncDownCalled {
		t.Errorf("syncDown failed")
	}

	// Get status
	res, err = mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if res != "get_status_res" || !getStatusCalled {
		t.Errorf("getStatus failed")
	}

	// Unknown tool
	_, err = mcp.CallTool(ctx, "unknown", nil)
	if err == nil || err.Error() != "unknown tool: unknown" {
		t.Errorf("Expected 'unknown tool: unknown', got %v", err)
	}
}
