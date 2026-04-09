package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	dir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	os.Setenv("OHC_WORKSPACE_DIR", dir)
	defer os.Unsetenv("OHC_WORKSPACE_DIR")
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridFSMCP()

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Write
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp test",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp test" {
		t.Fatalf("Expected 'mcp test', got '%v'", resMap["content"])
	}

	// List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]FileInfo)
	if len(entries) != 1 || entries[0].Name != "mcp.txt" {
		t.Fatalf("Unexpected entries: %v", entries)
	}
}
