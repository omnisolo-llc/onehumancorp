package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	mcpServer := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test ListTools
	tools := mcpServer.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Test CallTool - write_file
	writeArgs := map[string]interface{}{
		"target_path": "test.txt",
		"content":     "mcp content",
	}
	res, err := mcpServer.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Errorf("CallTool(write_file) failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}

	// Test CallTool - read_file
	readArgs := map[string]interface{}{
		"target_path": "test.txt",
	}
	res, err = mcpServer.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Errorf("CallTool(read_file) failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}
	if resMap["content"] != "mcp content" {
		t.Errorf("Expected 'mcp content', got '%v'", resMap["content"])
	}

	// Test CallTool - list_directory
	listArgs := map[string]interface{}{
		"target_path": ".",
	}
	res, err = mcpServer.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Errorf("CallTool(list_directory) failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test invalid tool
	_, err = mcpServer.CallTool(ctx, "invalid_tool", nil)
	if err == nil {
		t.Error("Expected error for invalid tool, got none")
	}

	// Test missing arguments
	_, err = mcpServer.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing arguments, got none")
	}
}
