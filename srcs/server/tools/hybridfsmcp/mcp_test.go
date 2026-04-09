package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong content: %v, %s", err, string(data))
	}

	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil || len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Fatalf("ListDir failed or wrong entries: %v, %v", err, entries)
	}

	err = provider.WriteFile(ctx, nil, "../test.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewCloudFSProvider(dir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org1"}

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong content: %v, %s", err, string(data))
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil || len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Fatalf("ListDir failed or wrong entries: %v, %v", err, entries)
	}

	err = provider.WriteFile(ctx, claims, "../test.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for traversal")
	}

	err = provider.WriteFile(ctx, nil, "test.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for missing claims")
	}
}

func TestHybridFSMCP(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools")
	}

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "a.txt", "content": "123"})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "a.txt"})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "123" {
		t.Fatalf("Wrong content")
	}

	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	files := resMap["files"].([]map[string]interface{})
	if len(files) != 1 || files[0]["name"] != "a.txt" {
		t.Fatalf("Wrong files")
	}

	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error for unknown tool")
	}
}
