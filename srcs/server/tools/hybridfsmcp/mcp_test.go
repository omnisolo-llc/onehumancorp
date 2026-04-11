package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"testing"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	provider := NewFileSystemProvider(true, "/tmp")
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewFileSystemProvider(true, tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test WriteFile tool
	contentStr := "test mcp content"
	contentB64 := base64.StdEncoding.EncodeToString([]byte(contentStr))

	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": contentB64,
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	writeResMap, ok := writeRes.(map[string]interface{})
	if !ok || writeResMap["status"] != "success" {
		t.Fatalf("write_file did not return success: %v", writeRes)
	}

	// Test ReadFile tool
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	readResMap, ok := readRes.(map[string]interface{})
	if !ok || readResMap["status"] != "success" {
		t.Fatalf("read_file did not return success: %v", readRes)
	}

	readB64, ok := readResMap["content"].(string)
	if !ok {
		t.Fatalf("read_file did not return base64 content string")
	}

	if readB64 != contentB64 {
		t.Fatalf("Expected %s, got %s", contentB64, readB64)
	}

	// Test ListDirectory tool
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	listResMap, ok := listRes.(map[string]interface{})
	if !ok || listResMap["status"] != "success" {
		t.Fatalf("list_directory did not return success: %v", listRes)
	}

	entries, ok := listResMap["entries"].([]string)
	if !ok {
		t.Fatalf("list_directory did not return entries array")
	}

	found := false
	for _, entry := range entries {
		if entry == "mcp_test.txt" {
			found = true
			break
		}
	}

	if !found {
		t.Fatalf("Expected to find mcp_test.txt in list_directory output: %v", entries)
	}
}
