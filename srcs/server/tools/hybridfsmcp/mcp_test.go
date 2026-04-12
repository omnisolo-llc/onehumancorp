package hybridfsmcp

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}

	toolNames := map[string]bool{}
	for _, tool := range tools {
		toolNames[tool.Name] = true
	}

	expected := []string{"read_file", "write_file", "list_directory", "search_files"}
	for _, name := range expected {
		if !toolNames[name] {
			t.Fatalf("Missing tool: %s", name)
		}
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if string(res.ResultData) != `{"content":"hello"}` {
		t.Fatalf("CallTool read_file bad result: %s", string(res.ResultData))
	}
}
