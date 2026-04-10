package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)

	ctx := context.Background()

	// WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if !reflect.DeepEqual(files, []string{"test.txt"}) {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}

	// Path traversal
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Errorf("Expected path traversal error, got none")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "world" {
		t.Errorf("Expected 'world', got '%s'", string(data))
	}

	// Check isolation
	tenantDir := filepath.Join(tmpDir, "tenant1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Errorf("File not written to tenant directory")
	}

	// No claims
	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims, got none")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider, true)

	ctx := context.Background()

	// CallTool write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp-test",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// CallTool read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	m := res.(map[string]interface{})
	if m["content"] != "mcp-test" {
		t.Errorf("Expected 'mcp-test', got '%v'", m["content"])
	}

	// CallTool list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	m = res.(map[string]interface{})
	files := m["files"].([]string)
	if !reflect.DeepEqual(files, []string{"mcp.txt"}) {
		t.Errorf("Expected ['mcp.txt'], got %v", files)
	}

	// Test unauth cloud
	mcpCloud := NewHybridFSMCP(provider, false)
	_, err = mcpCloud.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err == nil {
		t.Errorf("Expected unauthorized error, got none")
	}

	// Test tools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}
}
