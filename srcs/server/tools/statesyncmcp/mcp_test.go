package statesyncmcp

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// MockProvider implements StateSyncProvider for testing.
type MockProvider struct {
	SyncUpFunc    func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncDownFunc  func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	GetStatusFunc func(ctx context.Context, claims *auth.Claims) (*SyncStatus, error)
}

func (m *MockProvider) SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if m.SyncUpFunc != nil {
		return m.SyncUpFunc(ctx, claims)
	}
	return nil, nil
}

func (m *MockProvider) SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if m.SyncDownFunc != nil {
		return m.SyncDownFunc(ctx, claims)
	}
	return nil, nil
}

func (m *MockProvider) GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatus, error) {
	if m.GetStatusFunc != nil {
		return m.GetStatusFunc(ctx, claims)
	}
	return nil, nil
}

func TestStateSyncMCP_ListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&MockProvider{})
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	toolNames := map[string]bool{
		"sync_local_to_cloud": false,
		"sync_cloud_to_local": false,
		"get_sync_status":     false,
	}

	for _, tool := range tools {
		if _, ok := toolNames[tool.Name]; ok {
			toolNames[tool.Name] = true
		}
	}

	for name, found := range toolNames {
		if !found {
			t.Errorf("tool %s not found in list", name)
		}
	}
}

func TestStateSyncMCP_CallTool_SyncLocalToCloud(t *testing.T) {
	provider := &MockProvider{
		SyncUpFunc: func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
			return map[string]interface{}{"status": "success", "synced": 5}, nil
		},
	}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	result, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", result)
	}

	if resMap["status"] != "success" || resMap["synced"] != 5 {
		t.Errorf("unexpected result: %v", resMap)
	}
}

func TestStateSyncMCP_CallTool_SyncCloudToLocal(t *testing.T) {
	provider := &MockProvider{
		SyncDownFunc: func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
			return map[string]interface{}{"status": "success", "fetched": 3}, nil
		},
	}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	result, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", result)
	}

	if resMap["status"] != "success" || resMap["fetched"] != 3 {
		t.Errorf("unexpected result: %v", resMap)
	}
}

func TestStateSyncMCP_CallTool_GetSyncStatus(t *testing.T) {
	provider := &MockProvider{
		GetStatusFunc: func(ctx context.Context, claims *auth.Claims) (*SyncStatus, error) {
			return &SyncStatus{
				LastSyncTime: "2023-10-26T12:00:00Z",
				PendingItems: 10,
				Status:       "out_of_sync",
			}, nil
		},
	}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	result, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", result)
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}
	if resMap["last_sync_time"] != "2023-10-26T12:00:00Z" {
		t.Errorf("unexpected last_sync_time: %v", resMap["last_sync_time"])
	}
	if resMap["pending_items"] != 10 {
		t.Errorf("unexpected pending_items: %v", resMap["pending_items"])
	}
	if resMap["sync_state"] != "out_of_sync" {
		t.Errorf("unexpected sync_state: %v", resMap["sync_state"])
	}
}

func TestStateSyncMCP_CallTool_Errors(t *testing.T) {
	// Test unknown tool
	mcp := NewStateSyncMCP(&MockProvider{})
	_, err := mcp.CallTool(context.Background(), "unknown_tool", nil)
	if err == nil {
		t.Error("expected error for unknown tool")
	}

	// Test provider errors
	errProvider := &MockProvider{
		SyncUpFunc: func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
			return nil, errors.New("sync up error")
		},
		SyncDownFunc: func(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
			return nil, errors.New("sync down error")
		},
		GetStatusFunc: func(ctx context.Context, claims *auth.Claims) (*SyncStatus, error) {
			return nil, errors.New("get status error")
		},
	}
	errMcp := NewStateSyncMCP(errProvider)

	_, err = errMcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil || !errors.Is(err, errors.New("sync_local_to_cloud failed: sync up error")) && err.Error() != "sync_local_to_cloud failed: sync up error" {
		t.Errorf("expected sync up error, got %v", err)
	}

	_, err = errMcp.CallTool(context.Background(), "sync_cloud_to_local", nil)
	if err == nil || !errors.Is(err, errors.New("sync_cloud_to_local failed: sync down error")) && err.Error() != "sync_cloud_to_local failed: sync down error" {
		t.Errorf("expected sync down error, got %v", err)
	}

	_, err = errMcp.CallTool(context.Background(), "get_sync_status", nil)
	if err == nil || !errors.Is(err, errors.New("get_sync_status failed: get status error")) && err.Error() != "get_sync_status failed: get status error" {
		t.Errorf("expected get status error, got %v", err)
	}

	// Test nil provider
	nilMcp := NewStateSyncMCP(nil)
	_, err = nilMcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil {
		t.Error("expected error for nil provider")
	}
	_, err = nilMcp.CallTool(context.Background(), "sync_cloud_to_local", nil)
	if err == nil {
		t.Error("expected error for nil provider")
	}
	_, err = nilMcp.CallTool(context.Background(), "get_sync_status", nil)
	if err == nil {
		t.Error("expected error for nil provider")
	}
}
