package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	os.Setenv("OHC_WORKSPACE_DIR", tempDir)
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_WORKSPACE_DIR")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcpServer := NewHybridFSMCP()
	tools := mcpServer.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// write_file
	_, err := mcpServer.ExecuteTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file execution failed: %v", err)
	}

	// read_file
	res, err := mcpServer.ExecuteTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil || string(res.ResultData) != "world" {
		t.Fatalf("read_file failed or returned wrong data: %v, %s", err, string(res.ResultData))
	}

	// list_directory
	_, err = mcpServer.ExecuteTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}

	// search_files
	_, err = mcpServer.ExecuteTool(ctx, "search_files", map[string]interface{}{
		"path":    ".",
		"pattern": "hello",
	})
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}

	// Invalid args format
	_, err = mcpServer.ExecuteTool(ctx, "read_file", "bad_args")
	if err == nil {
		t.Fatalf("Expected invalid args format to fail")
	}

	// Unknown tool
	_, err = mcpServer.ExecuteTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected unknown tool to fail")
	}
}

func TestHybridFSMCPCloudContext(t *testing.T) {
	tempDir := t.TempDir()
	os.Setenv("OHC_TENANT_PV_DIR", tempDir)
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_TENANT_PV_DIR")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcpServer := NewHybridFSMCP()

	// Execute without claims (should fail)
	ctx := context.Background()
	_, err := mcpServer.ExecuteTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err == nil {
		t.Fatalf("Expected cloud execution without claims to fail")
	}

	// Execute with claims
	claims := &auth.Claims{OrganizationID: "tenant-2"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	_, err = mcpServer.ExecuteTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file with claims failed: %v", err)
	}
}
