package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcpServer := NewHybridFSMCP()
	tools := mcpServer.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_LocalProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "fsmcptest")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider := mcp.NewLocalFSProvider(dir)
	mcpServer := NewHybridFSMCPWithProvider(provider)
	ctx := context.Background()

	// Test WriteFile
	resWrite, err := mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello world",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if resWrite.(map[string]interface{})["status"] != "success" {
		t.Fatalf("Expected success, got %v", resWrite)
	}

	// Test ReadFile
	resRead, err := mcpServer.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if resRead.(map[string]interface{})["content"] != "hello world" {
		t.Fatalf("Expected 'hello world', got %v", resRead)
	}

	// Test ListDir
	resList, err := mcpServer.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := resList.(map[string]interface{})["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "test.txt" {
		t.Fatalf("Unexpected list_directory result: %v", resList)
	}
}

func TestHybridFSMCP_EnvironmentSetup(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_FS_ROOT", "/tmp/cloudfs")
	defer os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_FS_ROOT")

	mcpServer := NewHybridFSMCP()
	if mcpServer == nil {
		t.Fatalf("Failed to create HybridFSMCP")
	}

	// Cannot easily inspect the unexported provider type, but we can verify execution doesn't panic
	// and handles cloud scoping appropriately if we invoke it.

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	// To test properly, let's inject a temp dir and run a test
	dir, err := os.MkdirTemp("", "fsmcpcloudtest")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)
	os.Setenv("OHC_FS_ROOT", dir)

	mcpServerCloud := NewHybridFSMCP()
	_, err = mcpServerCloud.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "cloud file",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Verify it wrote to the tenant directory
	data, err := os.ReadFile(filepath.Join(dir, "org-123", "hello.txt"))
	if err != nil || string(data) != "cloud file" {
		t.Fatalf("Failed to verify cloud file written to correct tenant directory: %v, data: %s", err, string(data))
	}
}
