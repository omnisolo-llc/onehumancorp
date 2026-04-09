package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	if !provider.IsLocal() {
		t.Errorf("Expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("Expected %s, got %s", testContent, readContent)
	}

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../escaped.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "escapes workspace boundary") {
		t.Errorf("Expected path escape error, got %v", err)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test_file.txt" {
		t.Errorf("Unexpected ListDir results: %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || filepath.ToSlash(matches[0]) != "test_dir/test_file.txt" {
		t.Errorf("Unexpected SearchFiles results: %v", matches)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	if provider.IsLocal() {
		t.Errorf("Expected IsLocal to be false")
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant1"})

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("Expected %s, got %s", testContent, readContent)
	}

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../escaped.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "escapes tenant boundary") {
		t.Errorf("Expected path escape error, got %v", err)
	}

	// Test Unauthorized access (missing claims)
	err = provider.WriteFile(context.Background(), testPath, []byte("unauth"))
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got %v", err)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test_file.txt" {
		t.Errorf("Unexpected ListDir results: %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || filepath.ToSlash(matches[0]) != "test_dir/test_file.txt" {
		t.Errorf("Unexpected SearchFiles results: %v", matches)
	}
}

func TestHybridFSMCP_Server(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "true")
	server, err := NewHybridFSMCP(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create server: %v", err)
	}
	os.Unsetenv("OHC_STANDALONE")

	tools := server.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test write_file
	writeRes, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected status success")
	}

	// Test read_file
	readRes, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["content"] != "hello mcp" {
		t.Errorf("Unexpected read_file result")
	}

	// Test list_directory
	listRes, err := server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := listRes.(map[string]interface{})["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "test.txt" {
		t.Errorf("Unexpected list_directory result")
	}

	// Test search_files
	searchRes, err := server.CallTool(ctx, "search_files", map[string]interface{}{
		"dir":     ".",
		"pattern": "*.txt",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	matches := searchRes.(map[string]interface{})["matches"].([]string)
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("Unexpected search_files result")
	}
}

func TestHybridFSMCP_CloudMode(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_cloud_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "false")
	server, err := NewHybridFSMCP(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create server: %v", err)
	}
	os.Unsetenv("OHC_STANDALONE")

	ctxNoClaims := context.Background()
	_, err = server.CallTool(ctxNoClaims, "list_directory", map[string]interface{}{"path": "."})
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got %v", err)
	}

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant_cloud"})

	_, err = server.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	readRes, err := server.CallTool(ctxWithClaims, "read_file", map[string]interface{}{
		"path": "cloud.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["content"] != "cloud data" {
		t.Errorf("Unexpected read_file result")
	}
}
