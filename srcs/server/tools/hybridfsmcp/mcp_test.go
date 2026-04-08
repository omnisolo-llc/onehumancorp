package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	mcpServer := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test ListTools
	tools := mcpServer.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Test write_file
	writeArgs := map[string]interface{}{
		"path":    "hello.txt",
		"content": "hello mcp",
	}
	res, err := mcpServer.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("expected success response, got %v", res)
	}

	// Test read_file
	readArgs := map[string]interface{}{
		"path": "hello.txt",
	}
	res, err = mcpServer.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok || resMap["content"] != "hello mcp" {
		t.Errorf("expected content 'hello mcp', got %v", res)
	}

	// Test list_directory
	mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "dir/hello.txt",
		"content": "a",
	})
	listArgs := map[string]interface{}{
		"path": "dir",
	}
	res, err = mcpServer.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Errorf("invalid response format")
	}
	entries, ok := resMap["entries"].([]string)
	if !ok || len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("expected entry 'hello.txt', got %v", entries)
	}

	// Test invalid tool
	_, err = mcpServer.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for unknown tool")
	}
}
