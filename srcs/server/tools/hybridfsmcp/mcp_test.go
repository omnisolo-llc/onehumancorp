package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Unexpected dir entries: %v", entries)
	}

	// Test path traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Error("Expected error on path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "data.txt", []byte("cloud-data"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "cloud-data" {
		t.Errorf("Expected 'cloud-data', got '%s'", string(data))
	}

	// Verify file is under tenant dir
	if _, err := os.Stat(filepath.Join(tmpDir, "tenant-123", "data.txt")); os.IsNotExist(err) {
		t.Error("File was not written to tenant directory")
	}

	// Test without claims
	_, err = provider.ReadFile(context.Background(), "data.txt")
	if err == nil {
		t.Error("Expected error when missing claims")
	}
}

func TestHybridFSProxyMCP(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSProxyMCP(provider)
	ctx := context.Background()

	// Test write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.json",
		"data": "{}",
	})
	if err != nil {
		t.Fatalf("Failed write_file tool: %v", err)
	}

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.json",
	})
	if err != nil {
		t.Fatalf("Failed read_file tool: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["data"] != "{}" {
		t.Errorf("Expected data to be '{}', got %v", resMap["data"])
	}

	// Test list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("Failed list_directory tool: %v", err)
	}

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}
}