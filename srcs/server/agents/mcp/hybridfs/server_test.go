package hybridfs

import (
	"context"
	"testing"

)

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test ListTools
	tools := server.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// Test CallTool - write_file
	writeResRaw, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	})
	if err != nil {
		t.Errorf("unexpected error writing file via tool: %v", err)
	}

	writeRes, ok := writeResRaw.(map[string]interface{})
	if !ok || writeRes["success"] != true {
		t.Errorf("expected success=true, got %v", writeResRaw)
	}

	// Test CallTool - read_file
	readResRaw, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Errorf("unexpected error reading file via tool: %v", err)
	}

	readRes, ok := readResRaw.(map[string]interface{})
	if !ok || readRes["content"] != "mcp hello" {
		t.Errorf("expected content='mcp hello', got %v", readResRaw)
	}

	// Test CallTool - list_dir
	listResRaw, err := server.CallTool(ctx, "list_dir", map[string]interface{}{})
	if err != nil {
		t.Errorf("unexpected error listing dir via tool: %v", err)
	}

	listRes, ok := listResRaw.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", listResRaw)
	}

	entries, ok := listRes["entries"].([]string)
	if !ok || len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Errorf("expected entries=['mcp_test.txt'], got %v", listResRaw)
	}

	// Test CallTool - unknown tool
	_, err = server.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for unknown tool, got nil")
	}
}
