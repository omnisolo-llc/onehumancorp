package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Errorf("expected error on path traversal, got nil")
	}

	err = provider.WriteFile(ctx, "../evil.txt", []byte("evil"))
	if err == nil {
		t.Errorf("expected error on path traversal, got nil")
	}

	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Errorf("expected error on path traversal, got nil")
	}
}

func TestLocalFSProvider_Operations(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, "subdir/test2.txt", []byte("world"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	files, err := provider.ListDir(ctx, "subdir")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test2.txt" {
		t.Errorf("ListDir returned unexpected files: %v", files)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, "", "*.txt")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}

	expectedMatches := map[string]bool{
		"test.txt": false,
		"subdir/test2.txt": false,
	}
	for _, m := range matches {
		if _, ok := expectedMatches[m]; ok {
			expectedMatches[m] = true
		}
	}
	for k, v := range expectedMatches {
		if !v {
			t.Errorf("SearchFiles missing expected match: %s", k)
		}
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Unauthenticated
	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error when claims missing")
	}

	// Authenticated tenant1
	claims1 := &auth.Claims{OrganizationID: "tenant1"}
	ctx1 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims1)

	err = provider.WriteFile(ctx1, "data.txt", []byte("tenant1 data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data1, err := provider.ReadFile(ctx1, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data1) != "tenant1 data" {
		t.Errorf("expected 'tenant1 data', got '%s'", string(data1))
	}

	// Authenticated tenant2
	claims2 := &auth.Claims{OrganizationID: "tenant2"}
	ctx2 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims2)

	// tenant2 should not see tenant1's file
	_, err = provider.ReadFile(ctx2, "data.txt")
	if err == nil {
		t.Errorf("tenant2 read tenant1's file!")
	}

	err = provider.WriteFile(ctx2, "data.txt", []byte("tenant2 data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify SearchFiles strips prefix
	matches, err := provider.SearchFiles(ctx1, "", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "data.txt" {
		t.Errorf("expected [data.txt], got %v", matches)
	}
}

func TestHybridFSMCP_FactoryAndTools(t *testing.T) {
	tempDir := t.TempDir()

	// Test Cloud Mode (default)
	t.Setenv("OHC_STANDALONE", "false")
	mcpCloud, err := NewHybridFSMCP(tempDir)
	if err != nil {
		t.Fatalf("failed to create MCP: %v", err)
	}
	if mcpCloud.provider.IsLocal() {
		t.Errorf("expected cloud provider")
	}

	// Test Standalone Mode
	t.Setenv("OHC_STANDALONE", "true")
	mcpStandalone, err := NewHybridFSMCP(tempDir)
	if err != nil {
		t.Fatalf("failed to create MCP: %v", err)
	}
	if !mcpStandalone.provider.IsLocal() {
		t.Errorf("expected local provider")
	}

	// Test ListTools
	tools := mcpStandalone.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Test CallTool
	ctx := context.Background()
	res, err := mcpStandalone.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected write_file result: %v", resMap)
	}

	res, err = mcpStandalone.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["data"] != "hello" {
		t.Errorf("unexpected read_file result data: %v", resMap["data"])
	}

	res, err = mcpStandalone.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	res, err = mcpStandalone.CallTool(ctx, "search_files", map[string]interface{}{
		"path": "",
		"pattern": "*.txt",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}

	// Test Cloud CallTool auth required
	_, err = mcpCloud.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Errorf("expected auth error for cloud mcp")
	}
}

func TestHybridFSMCP_InvalidArguments(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("OHC_STANDALONE", "true")
	mcp, _ := NewHybridFSMCP(tempDir)

	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Errorf("expected error on unknown tool")
	}

	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error on missing args")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "a"})
	if err == nil {
		t.Errorf("expected error on missing args")
	}

	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": "a"})
	if err == nil {
		t.Errorf("expected error on missing args")
	}
}

func TestLocalFSProvider_DirectoryTraversalAdvanced(t *testing.T) {
	tempDir := t.TempDir()

	// Creating a directory ending in "foo"
	targetDir := filepath.Join(tempDir, "foo")
	os.MkdirAll(targetDir, 0755)

	// Creating a sibling directory "foobar"
	siblingDir := filepath.Join(tempDir, "foobar")
	os.MkdirAll(siblingDir, 0755)

	os.WriteFile(filepath.Join(siblingDir, "secret.txt"), []byte("secret"), 0644)

	provider, _ := NewLocalFSProvider(targetDir)
	ctx := context.Background()

	// Try to read the file in the sibling directory using a tricky path
	// E.g., if base is "/tmp/foo", and we pass "../foobar/secret.txt"
	// cleanPath will be "foobar/secret.txt"
	// filepath.Join will give "/tmp/foo/foobar/secret.txt"
	// which is safe.

	// But what if the path escaping wasn't caught?
	_, err := provider.ReadFile(ctx, "../foobar/secret.txt")
	if err == nil {
		t.Errorf("Expected error reading outside of bounded directory")
	}

	// Also check the SearchFiles edge cases
	_, err = provider.SearchFiles(ctx, "../", "*.txt")
	if err == nil {
		t.Errorf("Expected error searching outside of bounded directory")
	}
}
