package hybridfsmcp

import (
	"context"
	"encoding/json"
	"testing"
)

func TestMCPServer(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	// 1. Verify Tool Definitions
	tools := GetMCPTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	foundRead := false
	for _, tool := range tools {
		if tool["name"] == "read_file" {
			foundRead = true
		}
	}
	if !foundRead {
		t.Errorf("read_file tool not found in tool definitions")
	}

	// 2. Test CallTool - Write File
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "mcp.txt", Data: "mcp test"})
	_, err := server.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// 3. Test CallTool - Read File
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "mcp.txt"})
	res, err := server.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp test" {
		t.Errorf("Expected 'mcp test', got '%v'", resMap["content"])
	}

	// 4. Test CallTool - List Directory
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = server.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp.txt" {
		t.Errorf("Expected ['mcp.txt'], got %v", entries)
	}

	// 5. Test CallTool - Unknown Tool
	_, err = server.CallTool(ctx, "unknown_tool", []byte(`{}`))
	if err == nil {
		t.Errorf("Expected error for unknown tool, got nil")
	}
}
