package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	dir, _ := os.MkdirTemp("", "mcptools")
	defer os.RemoveAll(dir)
	mcp, _ := NewHybridFSMCP(dir)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_LocalCalls(t *testing.T) {
	dir, err := os.MkdirTemp("", "mcplocaltmp")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp, err := NewHybridFSMCP(dir)
	if err != nil {
		t.Fatalf("Failed to create MCP: %v", err)
	}

	ctx := context.Background()

	// Write
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Write base64
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":     "base64.txt",
		"content":  base64.StdEncoding.EncodeToString([]byte("b64 data")),
		"encoding": "base64",
	})
	if err != nil {
		t.Fatalf("CallTool write_file base64 failed: %v", err)
	}

	// Read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if res.(map[string]interface{})["content"] != "hello mcp" {
		t.Errorf("Unexpected read_file result: %v", res)
	}

	// List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := res.(map[string]interface{})["entries"].([]string)
	if len(entries) != 2 {
		t.Errorf("Expected 2 entries, got: %v", entries)
	}

	// Search
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"pattern": "*.txt",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	matches := res.(map[string]interface{})["matches"].([]string)
	if len(matches) != 2 {
		t.Errorf("Expected 2 match, got: %v", matches)
	}

	// Error bad args
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing path")
	}
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing path in write")
	}
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test"})
	if err == nil {
		t.Error("Expected error for missing content in write")
	}
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing path in list")
	}
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing pattern in search")
	}
	_, err = mcp.CallTool(ctx, "unknown", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for unknown tool")
	}
}

func TestHybridFSMCP_CloudCalls(t *testing.T) {
	dir, err := os.MkdirTemp("", "mcpcloudtmp")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp, err := NewHybridFSMCP(dir)
	if err != nil {
		t.Fatalf("Failed to create MCP: %v", err)
	}

	ctx := context.Background()

	// Missing claims should fail in CallTool due to early check
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != ErrUnauthorized {
		t.Errorf("Expected ErrUnauthorized, got: %v", err)
	}

	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-xyz",
	})

	_, err = mcp.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	res, err := mcp.CallTool(ctxWithClaims, "read_file", map[string]interface{}{
		"path": "cloud.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if res.(map[string]interface{})["content"] != "cloud data" {
		t.Errorf("Unexpected read_file result: %v", res)
	}
}
