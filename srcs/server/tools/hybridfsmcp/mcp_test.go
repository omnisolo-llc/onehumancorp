package hybridfsmcp

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	names := map[string]bool{}
	for _, tool := range tools {
		names[tool.Name] = true
	}

	for _, name := range []string{"read_file", "write_file", "list_directory"} {
		if !names[name] {
			t.Errorf("missing tool: %s", name)
		}
	}
}

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	_ = provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}

	if resMap["content"] != "hello" {
		t.Errorf("expected content 'hello', got %v", resMap["content"])
	}
}

func TestHybridFSMCP_CallTool_ReadFile_MissingPath(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider(t.TempDir()))
	_, err := mcp.CallTool(context.Background(), "read_file", map[string]interface{}{})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'path' argument") {
		t.Errorf("expected missing path error, got %v", err)
	}
}

func TestHybridFSMCP_CallTool_WriteFile(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello mcp",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}

	data, _ := provider.ReadFile(ctx, nil, "test.txt")
	if string(data) != "hello mcp" {
		t.Errorf("expected 'hello mcp', got '%s'", string(data))
	}
}

func TestHybridFSMCP_CallTool_WriteFile_MissingArgs(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider(t.TempDir()))

	_, err := mcp.CallTool(context.Background(), "write_file", map[string]interface{}{"content": "foo"})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'path' argument") {
		t.Errorf("expected missing path error, got %v", err)
	}

	_, err = mcp.CallTool(context.Background(), "write_file", map[string]interface{}{"path": "foo.txt"})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'content' argument") {
		t.Errorf("expected missing content error, got %v", err)
	}
}

func TestHybridFSMCP_CallTool_ListDirectory(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	_ = provider.WriteFile(ctx, nil, "file1.txt", []byte("1"))

	res, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map response")
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}

	items, ok := resMap["items"].([]map[string]interface{})
	if !ok || len(items) != 1 {
		t.Fatalf("expected 1 item in list")
	}

	if items[0]["name"] != "file1.txt" {
		t.Errorf("expected 'file1.txt', got %v", items[0]["name"])
	}
}

func TestHybridFSMCP_CallTool_ListDirectory_MissingPath(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider(t.TempDir()))
	_, err := mcp.CallTool(context.Background(), "list_directory", map[string]interface{}{})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'path' argument") {
		t.Errorf("expected missing path error, got %v", err)
	}
}

func TestHybridFSMCP_CallTool_Unknown(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider(t.TempDir()))
	_, err := mcp.CallTool(context.Background(), "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}
}

func TestHybridFSMCP_AuthClaimsForwarded(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-1"})

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatalf("unexpected error writing to cloud: %v", err)
	}

	// Read it back
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("unexpected error reading from cloud: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello cloud" {
		t.Errorf("expected content 'hello cloud', got %v", resMap["content"])
	}
}
