package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_Local(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "false")
	tmpDir := t.TempDir()
	t.Setenv("OHC_WORKSPACE_DIR", tmpDir)

	mcp := NewHybridFSMCP()

	ctx := context.Background()

	// Write file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello world",
	})
	if err != nil {
		t.Fatalf("expected no error writing file, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("expected no error reading file, got %v", err)
	}
	m := res.(map[string]interface{})
	if m["content"] != "hello world" {
		t.Errorf("expected 'hello world', got %v", m["content"])
	}

	// List directory
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("expected no error listing dir, got %v", err)
	}
	listM := resList.(map[string]interface{})
	results := listM["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "test.txt" {
		t.Errorf("expected 1 file named test.txt, got %v", results)
	}
}

func TestHybridFSMCP_Cloud(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	tmpDir := t.TempDir()
	t.Setenv("OHC_TENANT_PV_DIR", tmpDir)

	mcp := NewHybridFSMCP()

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test2.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatalf("expected no error writing file, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test2.txt",
	})
	if err != nil {
		t.Fatalf("expected no error reading file, got %v", err)
	}
	m := res.(map[string]interface{})
	if m["content"] != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %v", m["content"])
	}
}

func TestHybridFSMCP_InvalidTool(t *testing.T) {
	mcp := NewHybridFSMCP()
	_, err := mcp.CallTool(context.Background(), "invalid_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for invalid tool, got nil")
	}
}
