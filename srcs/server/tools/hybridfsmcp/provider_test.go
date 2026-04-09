package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_Bounds(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Test writing a file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("Failed to write file: %v", err)
	}

	// Test reading a file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("Failed to read file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test listing directory
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Unexpected dir entries: %v", entries)
	}

	// Test searching files
	matches, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Errorf("Failed to search files: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("Unexpected search matches: %v", matches)
	}

	// Test boundary escaping (absolute path)
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error reading absolute path")
	}

	// Test boundary escaping (directory traversal)
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Errorf("Expected error reading outside workspace")
	}
}

func TestCloudFSProvider_Bounds(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-123",
	})

	// Test writing a file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("Failed to write file: %v", err)
	}

	// Ensure the physical file exists in the correct tenant path
	physPath := filepath.Join(tempDir, "tenant-123", "test.txt")
	if _, err := os.Stat(physPath); os.IsNotExist(err) {
		t.Errorf("Expected file to exist at %s", physPath)
	}

	// Test boundary escaping (absolute path)
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error reading absolute path")
	}

	// Test boundary escaping (directory traversal)
	_, err = provider.ReadFile(ctx, "../tenant-456/test.txt")
	if err == nil {
		t.Errorf("Expected error reading outside tenant workspace")
	}

	// Test missing claims
	badCtx := context.Background()
	_, err = provider.ReadFile(badCtx, "test.txt")
	if err == nil {
		t.Errorf("Expected error reading without claims")
	}
}

func TestMCPServer_FactoryAndCall(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	// Test standalone
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_WORKSPACE_DIR", tempDir)
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	server, err := NewFileSystemMCPServer()
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Write
	_, err = server.Call(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	})
	if err != nil {
		t.Errorf("MCP Write failed: %v", err)
	}

	// Read
	res, err := server.Call(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Errorf("MCP Read failed: %v", err)
	}
	if res.ToolID != "read_file" {
		t.Errorf("Expected toolID read_file")
	}

	// List
	_, err = server.Call(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Errorf("MCP List failed: %v", err)
	}

	// Search
	_, err = server.Call(ctx, "search_files", map[string]interface{}{
		"directory": ".",
		"pattern":   "*.txt",
	})
	if err != nil {
		t.Errorf("MCP Search failed: %v", err)
	}

	// Error unknown tool
	_, err = server.Call(ctx, "unknown", nil)
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

func TestMCPServer_FactoryCloud(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-cloud-*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_VOLUME_ROOT", tempDir)
	defer os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_VOLUME_ROOT")

	server, err := NewFileSystemMCPServer()
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-cloud",
	})

	_, err = server.Call(ctx, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud hello",
	})
	if err != nil {
		t.Errorf("MCP Cloud Write failed: %v", err)
	}
}

func TestMCPServer_FactoryError(t *testing.T) {
	_, err := NewFileSystemMCPServer()
	if err == nil {
		t.Errorf("Expected error when no mode is specified")
	}
}
