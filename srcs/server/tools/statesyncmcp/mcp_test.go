package statesyncmcp

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	isLocal   bool
	syncUpErr error
	syncDownErr error
	getStatusErr error
}

func (m *mockProvider) SyncUp(ctx context.Context, tenantID string) (interface{}, error) {
	if m.syncUpErr != nil {
		return nil, m.syncUpErr
	}
	return map[string]interface{}{"status": "synced_up", "tenant": tenantID}, nil
}

func (m *mockProvider) SyncDown(ctx context.Context, tenantID string) (interface{}, error) {
	if m.syncDownErr != nil {
		return nil, m.syncDownErr
	}
	return map[string]interface{}{"status": "synced_down", "tenant": tenantID}, nil
}

func (m *mockProvider) GetStatus(ctx context.Context, tenantID string) (interface{}, error) {
	if m.getStatusErr != nil {
		return nil, m.getStatusErr
	}
	return map[string]interface{}{"status": "in_sync", "tenant": tenantID}, nil
}

func (m *mockProvider) IsLocal() bool {
	return m.isLocal
}

func TestListTools(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	names := make(map[string]bool)
	for _, tool := range tools {
		names[tool.Name] = true
	}

	expectedNames := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}
	for _, name := range expectedNames {
		if !names[name] {
			t.Errorf("missing tool: %s", name)
		}
	}
}

func TestCallTool_Unauthorized(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	// No claims in context
	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got: %v", err)
	}
}

func TestCallTool_CloudModeNoOp(t *testing.T) {
	provider := &mockProvider{isLocal: false}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if m["status"] != "success" || m["mode"] != "cloud" {
		t.Errorf("unexpected response: %v", m)
	}
}

func TestCallTool_SyncLocalToCloud(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok || m["status"] != "synced_up" || m["tenant"] != "org-123" {
		t.Errorf("unexpected response: %v", res)
	}
}

func TestCallTool_SyncCloudToLocal(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok || m["status"] != "synced_down" || m["tenant"] != "org-123" {
		t.Errorf("unexpected response: %v", res)
	}
}

func TestCallTool_GetSyncStatus(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok || m["status"] != "in_sync" || m["tenant"] != "org-123" {
		t.Errorf("unexpected response: %v", res)
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	provider := &mockProvider{isLocal: true}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil || err.Error() != "unknown tool: unknown_tool" {
		t.Errorf("expected unknown tool error, got: %v", err)
	}
}

func TestCallTool_ProviderError(t *testing.T) {
	expectedErr := errors.New("provider error")
	provider := &mockProvider{isLocal: true, syncUpErr: expectedErr}
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != expectedErr {
		t.Errorf("expected provider error, got: %v", err)
	}
}
