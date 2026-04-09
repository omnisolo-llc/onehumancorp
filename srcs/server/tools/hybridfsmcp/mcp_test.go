package hybridfsmcp

import (
	"context"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	// CallTool write_file
	writeArgs := map[string]interface{}{"path": "test.txt", "content": "test data"}
	_, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// CallTool read_file
	readArgs := map[string]interface{}{"path": "test.txt"}
	res, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "test data" {
		t.Fatalf("Content mismatch")
	}

	// CallTool list_directory
	listArgs := map[string]interface{}{"path": "."}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listRes := res.(map[string]interface{})
	entries := listRes["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("List mismatch")
	}

	// Unknown Tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error for unknown tool")
	}

	// Missing args
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error for missing args")
	}
}
