package mcp

import (
	"context"
	"os"
	"strings"
	"testing"
)

func TestFSMCPTools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "fs_mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	tools := DefaultFSTools(provider)
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// manually extract tools to test
	wt := &writeFileTool{provider: provider}
	rt := &readFileTool{provider: provider}
	lt := &readDirTool{provider: provider}

	ctx := context.Background()
	workDir := ""

	// Test write_file
	writeInput := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp content",
	}
	res, err := wt.Execute(ctx, workDir, writeInput)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if res != "Success" {
		t.Errorf("expected write_file to return 'Success', got %s", res)
	}

	// Test read_file
	readInput := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	res, err = rt.Execute(ctx, workDir, readInput)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if res != "mcp content" {
		t.Errorf("expected 'mcp content', got %s", res)
	}

	// Test list_directory
	listInput := map[string]interface{}{
		"path": ".",
	}
	res, err = lt.Execute(ctx, workDir, listInput)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}
	if !strings.Contains(res, "mcp_test.txt") {
		t.Errorf("expected list output to contain 'mcp_test.txt', got %s", res)
	}

	// Test missing args
	_, err = wt.Execute(ctx, workDir, map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error when args are missing")
	}

	// Test out of bounds via MCP tool
	escapeInput := map[string]interface{}{
		"path": "../escape_mcp.txt",
	}
	_, err = rt.Execute(ctx, workDir, escapeInput)
	if err == nil {
		t.Errorf("expected error when path escapes bounds via tool")
	}
}
