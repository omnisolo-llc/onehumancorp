package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	dir, _ := os.MkdirTemp("", "mcp")
	defer os.RemoveAll(dir)

	provider, _ := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "mcp"})
	if err != nil {
		t.Fatalf("WriteTool failed: %v", err)
	}

	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil || res.(map[string]interface{})["content"] != "mcp" {
		t.Fatalf("ReadTool failed: %v", err)
	}

	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil || len(res.(map[string]interface{})["entries"].([]map[string]interface{})) == 0 {
		t.Fatalf("ListDir failed: %v", err)
	}

	_, err = mcp.CallTool(ctx, "unknown", nil)
	if err == nil {
		t.Fatalf("Expected error for unknown tool")
	}

	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error for missing arguments")
	}
}
