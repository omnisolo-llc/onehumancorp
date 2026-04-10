package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello world")
	if err := provider.WriteFile(ctx, testPath, testContent); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, content)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected ListDir result: %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "test")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("unexpected SearchFiles result: %v", matches)
	}

	// Test Path Traversal
	if _, err := provider.ReadFile(ctx, "../outside.txt"); err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Create tenant dir
	tenantDir := filepath.Join(tmpDir, "tenant-1")
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		t.Fatalf("failed to create tenant dir: %v", err)
	}

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("cloud data")
	if err := provider.WriteFile(ctx, testPath, testContent); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, content)
	}

	// Test missing claims
	ctxNoClaims := context.Background()
	if _, err := provider.ReadFile(ctxNoClaims, testPath); err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Test cross-tenant access attempts (tenant-1 trying to access tenant-2 or root)
	if _, err := provider.ReadFile(ctx, "../tenant-2/file.txt"); err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	// Test Standalone Mode
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridFSMCP()

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Write file via tool
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Read file via tool
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp content" {
		t.Errorf("expected 'mcp content', got %v", resMap["content"])
	}

	// List directory via tool
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listMap := resList.(map[string]interface{})
	entries := listMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Errorf("unexpected list_directory entries: %v", entries)
	}

	// Search files via tool
	resSearch, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    ".",
		"pattern": "mcp",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	searchMap := resSearch.(map[string]interface{})
	matches := searchMap["matches"].([]string)
	if len(matches) != 1 || matches[0] != "mcp_test.txt" {
		t.Errorf("unexpected search_files matches: %v", matches)
	}
}

func TestLocalFSProvider_Errors(t *testing.T) {
	provider := NewLocalFSProvider("/nonexistent/base")
	ctx := context.Background()

	// ListDir error
	_, err := provider.ListDir(ctx, ".")
	if err == nil {
		t.Error("expected error for ListDir on nonexistent path")
	}

	// SearchFiles traversal error
	_, err = provider.SearchFiles(ctx, "../outside", "test")
	if err == nil {
		t.Error("expected error for SearchFiles traversal")
	}

	// ReadFile error
	_, err = provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Error("expected error for ReadFile on nonexistent path")
	}

	// WriteFile traversal error
	err = provider.WriteFile(ctx, "../outside.txt", []byte("data"))
	if err == nil {
		t.Error("expected error for WriteFile traversal")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_err")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// ListDir traversal error
	_, err = provider.ListDir(ctx, "../tenant-2")
	if err == nil {
		t.Error("expected error for ListDir traversal")
	}

	// ListDir error (dir does not exist)
	_, err = provider.ListDir(ctx, "nonexistent_dir")
	if err == nil {
		t.Error("expected error for ListDir on nonexistent dir")
	}

	// SearchFiles traversal error
	_, err = provider.SearchFiles(ctx, "../tenant-2", "test")
	if err == nil {
		t.Error("expected error for SearchFiles traversal")
	}

	// ReadFile error
	_, err = provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Error("expected error for ReadFile on nonexistent file")
	}

	// WriteFile traversal error
	err = provider.WriteFile(ctx, "../tenant-2/file.txt", []byte("data"))
	if err == nil {
		t.Error("expected error for WriteFile traversal")
	}

	// Test missing org ID
	ctxNoOrg := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})
	_, err = provider.getTenantBasePath(ctxNoOrg)
	if err == nil {
		t.Error("expected error for missing organization_id")
	}
}

func TestHybridFSMCP_Errors(t *testing.T) {
	mcp := NewHybridFSMCP()
	ctx := context.Background()

	// Missing path read_file
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path in read_file")
	}

	// Missing path write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path in write_file")
	}

	// Missing content write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Error("expected error for missing content in write_file")
	}

	// Missing path list_directory
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path in list_directory")
	}

	// Missing path search_files
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path in search_files")
	}

	// Missing pattern search_files
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": "."})
	if err == nil {
		t.Error("expected error for missing pattern in search_files")
	}

	// Unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for unknown tool")
	}
}

func TestHybridFSMCP_Errors_Provider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test_err")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridFSMCP()
	ctx := context.Background()

	// read_file error (file doesn't exist)
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "nonexistent.txt",
	})
	if err == nil {
		t.Error("expected error for read_file on nonexistent path")
	}

	// write_file error (path traversal)
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "../outside.txt",
		"content": "data",
	})
	if err == nil {
		t.Error("expected error for write_file traversal")
	}

	// list_directory error (dir doesn't exist)
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "nonexistent_dir",
	})
	if err == nil {
		t.Error("expected error for list_directory on nonexistent path")
	}

	// search_files error (path traversal)
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    "../outside",
		"pattern": "test",
	})
	if err == nil {
		t.Error("expected error for search_files traversal")
	}
}

func TestLocalFSProvider_MkdirError(t *testing.T) {
	// Note: It's hard to trigger MkdirAll error in a cross-platform way
	// without mocking os package, but we can try with a read-only dir if possible
	tmpDir, err := os.MkdirTemp("", "localfs_mkdir")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	err = os.Chmod(tmpDir, 0555) // Read and execute only
	if err != nil {
		t.Logf("Skipping MkdirError test: cannot chmod: %v", err)
		return
	}
	defer os.Chmod(tmpDir, 0755)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// WriteFile should fail to MkdirAll inside
	err = provider.WriteFile(ctx, "nested/file.txt", []byte("data"))
	if err == nil {
		t.Error("expected error for WriteFile in read-only dir")
	}
}

func TestCloudFSProvider_MkdirError(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_mkdir")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	tenantDir := filepath.Join(tmpDir, "tenant-1")
	if err := os.MkdirAll(tenantDir, 0555); err != nil {
		t.Fatalf("failed to create tenant dir: %v", err)
	}
	defer os.Chmod(tenantDir, 0755)

	err = provider.WriteFile(ctx, "nested/file.txt", []byte("data"))
	if err == nil {
		t.Error("expected error for WriteFile in read-only dir")
	}
}

func TestLocalFSProvider_WalkDirError(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_walk")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Create a nested dir
	nestedDir := filepath.Join(tmpDir, "nested")
	if err := os.MkdirAll(nestedDir, 0755); err != nil {
		t.Fatalf("failed to create nested dir: %v", err)
	}

	// Remove read permission from nested dir
	err = os.Chmod(nestedDir, 0333) // Write and execute only
	if err != nil {
		t.Logf("Skipping WalkDirError test: cannot chmod: %v", err)
		return
	}
	defer os.Chmod(nestedDir, 0755)

	// SearchFiles should fail when traversing the nested dir
	_, err = provider.SearchFiles(ctx, ".", "test")
	if err == nil {
		t.Error("expected error for SearchFiles traversal with unreadable dir")
	}
}

func TestCloudFSProvider_WalkDirError(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_walk")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	tenantDir := filepath.Join(tmpDir, "tenant-1")
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		t.Fatalf("failed to create tenant dir: %v", err)
	}

	nestedDir := filepath.Join(tenantDir, "nested")
	if err := os.MkdirAll(nestedDir, 0755); err != nil {
		t.Fatalf("failed to create nested dir: %v", err)
	}

	err = os.Chmod(nestedDir, 0333)
	if err != nil {
		t.Logf("Skipping WalkDirError test: cannot chmod: %v", err)
		return
	}
	defer os.Chmod(nestedDir, 0755)

	_, err = provider.SearchFiles(ctx, ".", "test")
	if err == nil {
		t.Error("expected error for SearchFiles traversal with unreadable dir")
	}
}
