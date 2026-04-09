package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestHybridFSMCP_Tools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// 1. Test WriteFile
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("expected success status from write_file")
	}

	// 2. Test ReadFile
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}

	resMap, ok = res.(map[string]interface{})
	if !ok || resMap["status"] != "success" || resMap["content"] != "world" {
		t.Errorf("expected content 'world' from read_file, got: %v", resMap)
	}

	// 3. Test ListDir
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}

	resMap, ok = res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("expected success status from list_directory")
	}

	files, ok := resMap["files"].([]map[string]interface{})
	if !ok || len(files) == 0 {
		t.Errorf("expected files in list_directory output")
	}

	found := false
	for _, f := range files {
		if f["name"] == "hello.txt" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected to find hello.txt in list_directory")
	}
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := NewHybridFSMCP(nil)
	tools := mcp.ListTools()
	if len(tools) < 3 {
		t.Errorf("expected at least 3 tools, got %d", len(tools))
	}
}
