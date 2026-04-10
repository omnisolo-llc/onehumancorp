package hybridfsmcp

import (
	"context"
	"testing"
)

func TestHybridFSMCP(t *testing.T) {
	dir := t.TempDir()
	p := NewProvider(true, dir)
	mcp := NewHybridFSMCP(p)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()
	_, err := mcp.CallTool(ctx, nil, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello mcp",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	res, err := mcp.CallTool(ctx, nil, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res.(string) != "hello mcp" {
		t.Errorf("expected 'hello mcp', got '%s'", res)
	}

	dirRes, err := mcp.CallTool(ctx, nil, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	entries := dirRes.([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	_, err = mcp.CallTool(ctx, nil, "unknown", nil)
	if err == nil {
		t.Error("expected error for unknown tool, got nil")
	}
}
