package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func setupTestDir(t *testing.T) (string, func()) {
	dir, err := os.MkdirTemp("", "mcp_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}

	cleanup := func() {
		os.RemoveAll(dir)
	}
	return dir, cleanup
}

func TestLocalFSProvider(t *testing.T) {
	workspace, cleanup := setupTestDir(t)
	defer cleanup()

	provider := NewLocalFSProvider(workspace)
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, nil, "dir/test2.txt", []byte("hello again"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	files, err := provider.ListDir(ctx, nil, "dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test2.txt" {
		t.Errorf("ListDir returned unexpected result: %v", files)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal detected") {
		t.Errorf("Expected path traversal error, got: %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseStorage, cleanup := setupTestDir(t)
	defer cleanup()

	provider := NewCloudFSProvider(baseStorage)
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure tenant isolation
	if _, err := os.Stat(filepath.Join(baseStorage, "tenant1", "test.txt")); os.IsNotExist(err) {
		t.Errorf("File was not created in tenant directory")
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, claims, "dir/test2.txt", []byte("hello again"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	files, err := provider.ListDir(ctx, claims, "dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test2.txt" {
		t.Errorf("ListDir returned unexpected result: %v", files)
	}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("data"))
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got: %v", err)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, claims, "../tenant2/test.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal detected") {
		t.Errorf("Expected path traversal error, got: %v", err)
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	workspace, cleanup := setupTestDir(t)
	defer cleanup()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_WORKSPACE_DIR", workspace)
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	mcp := NewHybridFSMCP()
	ctx := context.Background()

	// Test tools listing
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Test CallTool - write_file
	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if status := writeRes.(map[string]interface{})["status"]; status != "success" {
		t.Errorf("Expected status success, got %v", status)
	}

	// Test CallTool - read_file
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if data := readRes.(map[string]interface{})["data"]; data != "hello mcp" {
		t.Errorf("Expected data 'hello mcp', got %v", data)
	}

	// Test CallTool - list_directory
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	files := listRes.(map[string]interface{})["files"].([]string)
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}

	// Test Unknown Tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil || !strings.Contains(err.Error(), "unknown tool") {
		t.Errorf("Expected unknown tool error, got: %v", err)
	}
}

func TestHybridFSMCP_CloudMode(t *testing.T) {
	baseStorage, cleanup := setupTestDir(t)
	defer cleanup()

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_BASE_STORAGE_DIR", baseStorage)
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_BASE_STORAGE_DIR")

	mcp := NewHybridFSMCP()
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test CallTool - write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello cloud mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Verify isolation
	if _, err := os.Stat(filepath.Join(baseStorage, "tenant1", "test.txt")); os.IsNotExist(err) {
		t.Errorf("File was not created in tenant directory")
	}

	// Test CallTool - read_file
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if data := readRes.(map[string]interface{})["data"]; data != "hello cloud mcp" {
		t.Errorf("Expected data 'hello cloud mcp', got %v", data)
	}
}
