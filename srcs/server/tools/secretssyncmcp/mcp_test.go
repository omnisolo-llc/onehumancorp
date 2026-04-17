package secretssyncmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	syncDownResult map[string]interface{}
	syncDownErr    error
	syncUpResult   map[string]interface{}
	syncUpErr      error
}

func (m *mockProvider) SyncSecretsDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.syncDownResult, m.syncDownErr
}

func (m *mockProvider) SyncSecretsUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.syncUpResult, m.syncUpErr
}

func TestListTools(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, true)
	tools := mcp.ListTools()

	if len(tools) != 2 {
		t.Fatalf("expected 2 tools, got %d", len(tools))
	}

	names := map[string]bool{}
	for _, tool := range tools {
		names[tool.Name] = true
	}

	for _, name := range []string{"sync_secrets_down", "sync_secrets_up"} {
		if !names[name] {
			t.Errorf("missing tool: %s", name)
		}
	}
}

func TestCallTool_NotLocal(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, false)
	res, err := mcp.CallTool(context.Background(), "sync_secrets_down", nil)
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
	mcp := NewSecretsSyncMCP(&mockProvider{}, true)
	_, err := mcp.CallTool(context.Background(), "sync_secrets_down", nil)
	if err == nil {
		t.Fatalf("expected error due to missing claims")
	}
}

func TestCallTool_SyncSecretsDown(t *testing.T) {
	expectedRes := map[string]interface{}{"status": "success"}
	provider := &mockProvider{
		syncDownResult: expectedRes,
	}

	mcp := NewSecretsSyncMCP(provider, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "sync_secrets_down", nil)
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

func TestCallTool_SyncSecretsUp(t *testing.T) {
	expectedRes := map[string]interface{}{"status": "success", "synced_count": 5}
	provider := &mockProvider{
		syncUpResult: expectedRes,
	}

	mcp := NewSecretsSyncMCP(provider, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "sync_secrets_up", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["synced_count"] != 5 {
		t.Errorf("expected synced_count 5, got %v", resMap["synced_count"])
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	mcp := NewSecretsSyncMCP(&mockProvider{}, true)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}
}
