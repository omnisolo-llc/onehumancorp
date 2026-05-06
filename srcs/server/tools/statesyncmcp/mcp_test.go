package statesyncmcp

import (
	"context"
	"errors"
	"reflect"
	"testing"
)

type MockProvider struct {
	syncUpResult   int
	syncUpErr      error
	syncDownResult int
	syncDownErr    error
	statusResult   *SyncStatus
	statusErr      error
}

func (m *MockProvider) SyncUp(ctx context.Context, orgID string) (int, error) {
	return m.syncUpResult, m.syncUpErr
}

func (m *MockProvider) SyncDown(ctx context.Context, orgID string) (int, error) {
	return m.syncDownResult, m.syncDownErr
}

func (m *MockProvider) GetStatus(ctx context.Context, orgID string) (*SyncStatus, error) {
	return m.statusResult, m.statusErr
}

type MockHub struct {
	provider StateSyncProvider
}

func (m *MockHub) StateSync() StateSyncProvider {
	return m.provider
}

func TestListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&MockHub{})
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	names := map[string]bool{}
	for _, tool := range tools {
		names[tool["name"].(string)] = true
	}

	expected := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}
	for _, e := range expected {
		if !names[e] {
			t.Errorf("missing tool: %s", e)
		}
	}
}

func TestCallTool_MissingClaims(t *testing.T) {
	mcp := NewStateSyncMCP(&MockHub{})
	_, err := mcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "unauthorized: missing claims or organization ID" {
		t.Errorf("unexpected error: %v", err)
	}

	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{})
	_, err = mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCallTool_MissingProvider(t *testing.T) {
	mcp := NewStateSyncMCP(&MockHub{provider: nil})
	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{OrganizationID: "org-1"})
	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "state sync provider not configured" {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	mcp := NewStateSyncMCP(&MockHub{provider: &MockProvider{}})
	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{OrganizationID: "org-1"})
	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "unknown tool: unknown_tool" {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestCallTool_SyncLocalToCloud(t *testing.T) {
	mockProvider := &MockProvider{syncUpResult: 5}
	mcp := NewStateSyncMCP(&MockHub{provider: mockProvider})
	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{OrganizationID: "org-1"})

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	if resMap["synced_count"] != 5 || resMap["status"] != "success" {
		t.Errorf("unexpected result: %v", resMap)
	}

	mockProvider.syncUpErr = errors.New("up error")
	_, err = mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil || err.Error() != "sync_local_to_cloud failed: up error" {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestCallTool_SyncCloudToLocal(t *testing.T) {
	mockProvider := &MockProvider{syncDownResult: 3}
	mcp := NewStateSyncMCP(&MockHub{provider: mockProvider})
	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{OrganizationID: "org-1"})

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	if resMap["synced_count"] != 3 || resMap["status"] != "success" {
		t.Errorf("unexpected result: %v", resMap)
	}

	mockProvider.syncDownErr = errors.New("down error")
	_, err = mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil || err.Error() != "sync_cloud_to_local failed: down error" {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestCallTool_GetSyncStatus(t *testing.T) {
	expectedStatus := &SyncStatus{LastSyncTime: "2023-01-01T00:00:00Z", PendingTasks: 2, Status: "synced"}
	mockProvider := &MockProvider{statusResult: expectedStatus}
	mcp := NewStateSyncMCP(&MockHub{provider: mockProvider})
	ctx := context.WithValue(context.Background(), ContextKeyClaims, &Claims{OrganizationID: "org-1"})

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !reflect.DeepEqual(res, expectedStatus) {
		t.Errorf("expected %v, got %v", expectedStatus, res)
	}

	mockProvider.statusErr = errors.New("status error")
	_, err = mcp.CallTool(ctx, "get_sync_status", nil)
	if err == nil || err.Error() != "get_sync_status failed: status error" {
		t.Errorf("unexpected error: %v", err)
	}
}
