package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	readCalled  bool
	writeCalled bool
	listCalled  bool
}

func (m *mockProvider) ReadFile(ctx context.Context, path string, claims *auth.Claims) (map[string]interface{}, error) {
	m.readCalled = true
	return map[string]interface{}{"status": "success", "content": "mock content"}, nil
}

func (m *mockProvider) WriteFile(ctx context.Context, path string, content string, claims *auth.Claims) (map[string]interface{}, error) {
	m.writeCalled = true
	return map[string]interface{}{"status": "success", "message": "mock written"}, nil
}

func (m *mockProvider) ListDir(ctx context.Context, path string, claims *auth.Claims) (map[string]interface{}, error) {
	m.listCalled = true
	return map[string]interface{}{"status": "success", "files": []map[string]interface{}{}}, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := &HybridFSMCP{provider: &mockProvider{}}
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	expectedNames := map[string]bool{
		"read_file":      true,
		"write_file":     true,
		"list_directory": true,
	}

	for _, tool := range tools {
		if !expectedNames[tool.Name] {
			t.Errorf("unexpected tool name: %s", tool.Name)
		}
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	provider := &mockProvider{}
	mcp := &HybridFSMCP{provider: provider}
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test read_file
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !provider.readCalled {
		t.Error("expected ReadFile to be called")
	}

	// Test write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "data"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !provider.writeCalled {
		t.Error("expected WriteFile to be called")
	}

	// Test list_directory
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "test_dir"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !provider.listCalled {
		t.Error("expected ListDir to be called")
	}

	// Test unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for unknown tool")
	}
}

func TestNewHybridFSMCP(t *testing.T) {
	// Test Local
	localMCP := NewHybridFSMCP(true, "/tmp/local")
	if _, ok := localMCP.provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider, got %T", localMCP.provider)
	}

	// Test Cloud
	cloudMCP := NewHybridFSMCP(false, "/tmp/cloud")
	if _, ok := cloudMCP.provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider, got %T", cloudMCP.provider)
	}
}
