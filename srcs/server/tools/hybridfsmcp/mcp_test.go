package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected error for directory traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write
	err = provider.WriteFile(ctx, "tenant_test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Verify isolation on disk
	content, err := os.ReadFile(filepath.Join(tmpDir, "tenant1", "tenant_test.txt"))
	if err != nil {
		t.Fatalf("Failed to read actual file: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(content))
	}

	// Test read
	data, err := provider.ReadFile(ctx, "tenant_test.txt")
	if err != nil {
		t.Fatalf("Failed to read file via provider: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "tenant_test.txt" {
		t.Errorf("Expected ['tenant_test.txt'], got %v", entries)
	}

	// Test unauthorized (missing claims)
	_, err = provider.ReadFile(context.Background(), "tenant_test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims")
	}

	// Test traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected error for directory traversal")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcpfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// CallTool write_file
	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected success, got %v", writeRes)
	}

	// CallTool read_file
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["content"] != "mcp hello" {
		t.Errorf("Expected 'mcp hello', got %v", readRes)
	}

	// CallTool list_directory
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := listRes.(map[string]interface{})["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Errorf("Expected ['mcp_test.txt'], got %v", entries)
	}

	// Unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}

	// Missing args
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path")
	}
}
