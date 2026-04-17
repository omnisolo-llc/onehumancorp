package secretssyncmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	syncDownCalled bool
	syncUpCalled   bool
}

func (m *mockProvider) SyncSecretsDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	m.syncDownCalled = true
	return map[string]interface{}{"status": "success"}, nil
}

func (m *mockProvider) SyncSecretsUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	m.syncUpCalled = true
	return map[string]interface{}{"status": "success"}, nil
}

func TestListTools_Local(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, true)
	tools := mcp.ListTools()
	if len(tools) != 2 {
		t.Fatalf("expected 2 tools, got %d", len(tools))
	}
	if tools[0].Name != "secrets_sync_down" {
		t.Errorf("expected secrets_sync_down, got %s", tools[0].Name)
	}
}

func TestListTools_Cloud(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, false)
	tools := mcp.ListTools()
	if len(tools) != 0 {
		t.Fatalf("expected 0 tools, got %d", len(tools))
	}
}

func TestCallTool_SyncSecretsDown(t *testing.T) {
	provider := &mockProvider{}
	mcp := NewSecretsSyncMCP(provider, true)

	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "secrets_sync_down", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !provider.syncDownCalled {
		t.Error("expected SyncSecretsDown to be called")
	}
}

func TestCallTool_SyncSecretsUp(t *testing.T) {
	provider := &mockProvider{}
	mcp := NewSecretsSyncMCP(provider, true)

	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "secrets_sync_up", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !provider.syncUpCalled {
		t.Error("expected SyncSecretsUp to be called")
	}
}

func TestCallTool_Unknown(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, true)
	_, err := mcp.CallTool(context.Background(), "unknown", nil)
	if err == nil {
		t.Error("expected error for unknown tool")
	}
}

func TestCallTool_Cloud(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, false)
	_, err := mcp.CallTool(context.Background(), "secrets_sync_down", nil)
	if err == nil {
		t.Error("expected error when calling tool in cloud mode")
	}
}
