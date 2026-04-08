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
	claims := &auth.Claims{} // Not strictly needed for local, but passed

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, claims, "dir/file2.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile (dir) failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("Expected 2 entries, got %d", len(entries))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, claims, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error when escaping bounds, got nil")
	}

	// Test SearchFiles
	res, err := provider.SearchFiles(ctx, claims, ".", "file2")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(res) != 1 || res[0] != "dir/file2.txt" {
		t.Errorf("Expected ['dir/file2.txt'], got %v", res)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	ctx := context.Background()
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("tenant_data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it wrote to the correct tenant directory
	tenantDir := filepath.Join(tmpDir, "tenant1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Errorf("File not found in tenant directory")
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "tenant_data" {
		t.Errorf("Expected 'tenant_data', got '%s'", string(data))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, claims, "../tenant2/escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error when escaping tenant bounds, got nil")
	}

	// Test no claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error when claims are nil, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Write file via MCP
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "mcp_world",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if res.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected success, got %v", res)
	}

	// Read file via MCP
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if res.(map[string]interface{})["content"] != "mcp_world" {
		t.Errorf("Expected 'mcp_world', got %v", res.(map[string]interface{})["content"])
	}

	// List directory via MCP
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	files := res.(map[string]interface{})["files"].([]string)
	if len(files) != 1 || files[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", files)
	}

	// Search files via MCP
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path": ".",
		"pattern": "hello",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	files = res.(map[string]interface{})["files"].([]string)
	if len(files) != 1 || files[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", files)
	}
}

func TestFactory(t *testing.T) {
	// Test Cloud mode (default)
	os.Setenv("OHC_STANDALONE", "false")
	provider := NewProvider()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", provider)
	}

	// Test Standalone mode
	os.Setenv("OHC_STANDALONE", "true")
	provider = NewProvider()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", provider)
	}
}
