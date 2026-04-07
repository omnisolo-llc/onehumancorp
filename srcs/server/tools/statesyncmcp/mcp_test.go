package statesyncmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	syncUpCalled   bool
	syncDownCalled bool
	getStatusCalled bool
}

func (m *mockProvider) SyncUp(ctx context.Context, claims *auth.Claims) (SyncResult, error) {
	m.syncUpCalled = true
	return SyncResult{SyncedCount: 1}, nil
}

func (m *mockProvider) SyncDown(ctx context.Context, claims *auth.Claims) (SyncResult, error) {
	m.syncDownCalled = true
	return SyncResult{SyncedCount: 2}, nil
}

func (m *mockProvider) GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	m.getStatusCalled = true
	return map[string]interface{}{"status": "ok"}, nil
}

func TestStateSyncMCP_ListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&mockProvider{})
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestStateSyncMCP_CallTool(t *testing.T) {
	mockProv := &mockProvider{}
	mcp := NewStateSyncMCP(mockProv)

	// Missing claims
	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", map[string]interface{}{})
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Fatalf("Expected missing claims error, got %v", err)
	}

	// Valid claims
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})

	// Sync local to cloud
	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", map[string]interface{}{})
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	syncRes := res.(SyncResult)
	if syncRes.SyncedCount != 1 || !mockProv.syncUpCalled {
		t.Fatalf("Expected syncUp to be called and return 1, got %v", syncRes)
	}

	// Sync cloud to local
	res, err = mcp.CallTool(ctx, "sync_cloud_to_local", map[string]interface{}{})
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	syncRes = res.(SyncResult)
	if syncRes.SyncedCount != 2 || !mockProv.syncDownCalled {
		t.Fatalf("Expected syncDown to be called and return 2, got %v", syncRes)
	}

	// Get status
	res, err = mcp.CallTool(ctx, "get_sync_status", map[string]interface{}{})
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	statusRes := res.(map[string]interface{})
	if statusRes["status"] != "ok" || !mockProv.getStatusCalled {
		t.Fatalf("Expected getStatus to be called and return ok, got %v", statusRes)
	}

	// Unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil || err.Error() != "unknown tool: unknown_tool" {
		t.Fatalf("Expected unknown tool error, got %v", err)
	}
}
