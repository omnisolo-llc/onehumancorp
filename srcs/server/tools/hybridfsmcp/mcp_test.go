package hybridfsmcp

import (
	"context"
	"strings"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	t.Run("ListTools", func(t *testing.T) {
		tools := mcp.ListTools()
		if len(tools) != 3 {
			t.Errorf("Expected 3 tools, got %d", len(tools))
		}

		toolNames := make(map[string]bool)
		for _, tool := range tools {
			toolNames[tool.Name] = true
		}

		if !toolNames["read_file"] || !toolNames["write_file"] || !toolNames["list_directory"] {
			t.Errorf("Missing expected tools")
		}
	})

	t.Run("CallTool Write and Read", func(t *testing.T) {
		// Write file
		writeArgs := map[string]interface{}{
			"path":    "test.txt",
			"content": "mcp content",
		}
		writeRes, err := mcp.CallTool(ctx, "write_file", writeArgs)
		if err != nil {
			t.Fatalf("write_file failed: %v", err)
		}

		writeMap, ok := writeRes.(map[string]interface{})
		if !ok || writeMap["status"] != "success" {
			t.Errorf("write_file unexpected response: %v", writeRes)
		}

		// Read file
		readArgs := map[string]interface{}{
			"path": "test.txt",
		}
		readRes, err := mcp.CallTool(ctx, "read_file", readArgs)
		if err != nil {
			t.Fatalf("read_file failed: %v", err)
		}

		readMap, ok := readRes.(map[string]interface{})
		if !ok || readMap["content"] != "mcp content" {
			t.Errorf("read_file unexpected response: %v", readRes)
		}
	})

	t.Run("CallTool List Directory", func(t *testing.T) {
		listArgs := map[string]interface{}{
			"path": ".",
		}
		listRes, err := mcp.CallTool(ctx, "list_directory", listArgs)
		if err != nil {
			t.Fatalf("list_directory failed: %v", err)
		}

		listMap, ok := listRes.(map[string]interface{})
		if !ok || listMap["status"] != "success" {
			t.Errorf("list_directory unexpected response: %v", listRes)
		}

		entries, ok := listMap["entries"].([]map[string]interface{})
		if !ok || len(entries) == 0 {
			t.Errorf("list_directory expected entries, got: %v", listRes)
		}
	})

	t.Run("Unknown Tool", func(t *testing.T) {
		_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
		if err == nil {
			t.Errorf("Expected error for unknown tool, got nil")
		}
		if !strings.Contains(err.Error(), "unknown tool") {
			t.Errorf("Expected 'unknown tool' error message, got: %v", err)
		}
	})

	t.Run("Missing Arguments", func(t *testing.T) {
		_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
		if err == nil {
			t.Errorf("Expected error for missing arguments, got nil")
		}
	})
}
