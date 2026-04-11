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
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Absolute Path Rejection
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error for absolute path")
	}

	// Test Path Traversal Rejection
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Errorf("Expected error for path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Create context without claims
	ctxNoClaims := context.Background()

	// Test ReadFile without claims
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err == nil {
		t.Errorf("Expected error when missing claims")
	}

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to the correct tenant dir
	tenantDirPath := filepath.Join(tmpDir, "tenant-123")
	if _, err := os.Stat(tenantDirPath); os.IsNotExist(err) {
		t.Fatalf("Tenant directory was not created: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}
}

func TestHybridFSMCP_Standalone(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybrid_mcp_standalone")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_FS_ROOT")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp, err := NewHybridFSMCP()
	if err != nil {
		t.Fatalf("NewHybridFSMCP failed: %v", err)
	}

	ctx := context.Background()

	// Test Write
	resWrite, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "test data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	writeMap := resWrite.(map[string]interface{})
	if writeMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", writeMap["status"])
	}

	// Test Read
	resRead, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	readMap := resRead.(map[string]interface{})
	if readMap["content"] != "test data" {
		t.Errorf("Expected content 'test data', got %v", readMap["content"])
	}

	// Test List Directory
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	listMap := resList.(map[string]interface{})
	entries := listMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Errorf("Expected ['mcp_test.txt'], got %v", entries)
	}

	// Test Unknown Tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}

	// Test ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	expectedNames := []string{"read_file", "write_file", "list_directory"}
	for i, tool := range tools {
		if tool.Name != expectedNames[i] {
			t.Errorf("Expected tool %d to be %s, got %s", i, expectedNames[i], tool.Name)
		}
	}
}

func TestHybridFSMCP_Cloud(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybrid_mcp_cloud")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_FS_ROOT")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp, err := NewHybridFSMCP()
	if err != nil {
		t.Fatalf("NewHybridFSMCP failed: %v", err)
	}

	// Create context with claims
	claims := &auth.Claims{
		OrganizationID: "org-456",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)
	ctxNoClaims := context.Background()

	// Test CallTool without claims should fail
	_, err = mcp.CallTool(ctxNoClaims, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err == nil {
		t.Errorf("Expected unauthorized error")
	}

	// Test CallTool with claims should succeed
	resWrite, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "cloud_test.txt",
		"content": "cloud data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	if !reflect.DeepEqual(resWrite, map[string]interface{}{"status": "success"}) {
		t.Errorf("Unexpected write result: %v", resWrite)
	}
}
