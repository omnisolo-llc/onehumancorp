package hybridfsmcp

import (
	"context"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "hello"})
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if res.(string) != "hello" {
		t.Fatalf("expected 'hello', got %v", res)
	}

	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "../outside.txt"})
	if err == nil {
		t.Fatalf("expected error reading outside bounds")
	}
}
