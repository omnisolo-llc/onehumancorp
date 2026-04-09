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
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Boundary Enforcements
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Error("Expected error when escaping base directory, got nil")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error when escaping base directory, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create a context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "cloud.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify file is correctly scoped to tenant directory on disk
	onDiskPath := filepath.Join(tmpDir, "tenant-123", "cloud.txt")
	if _, err := os.Stat(onDiskPath); os.IsNotExist(err) {
		t.Errorf("File was not written to the tenant-scoped directory: %s", onDiskPath)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "cloud.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "cloud.txt" {
		t.Errorf("Expected ['cloud.txt'], got %v", entries)
	}

	// Test Boundary Enforcements
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Error("Expected error when escaping tenant directory, got nil")
	}

	// Test Missing Claims
	ctxNoClaims := context.Background()
	err = provider.WriteFile(ctxNoClaims, "test.txt", []byte("fail"))
	if err == nil {
		t.Error("Expected error with missing claims, got nil")
	}
}

func TestHybridFSMCP_LocalMode(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_local_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	t.Setenv("OHC_STANDALONE", "true")

	mcp := NewHybridFSMCP(tmpDir)
	ctx := context.Background()

	// Test write_file tool
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["status"] != "success" {
		t.Errorf("write_file unexpected result: %v", res)
	}

	// Test read_file tool
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["content"] != "mcp content" {
		t.Errorf("read_file unexpected result: %v", res)
	}

	// Test list_directory tool
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok {
		t.Errorf("list_directory unexpected result format: %v", res)
	} else {
		entries, _ := resMap["entries"].([]string)
		if len(entries) != 1 || entries[0] != "mcp_test.txt" {
			t.Errorf("list_directory unexpected entries: %v", entries)
		}
	}

	// Test search_files tool
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    ".",
		"pattern": "mcp_test",
	})
	if err != nil {
		t.Fatalf("search_files tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok {
		t.Errorf("search_files unexpected result format: %v", res)
	} else {
		matches, _ := resMap["matches"].([]string)
		if len(matches) != 1 || matches[0] != "mcp_test.txt" {
			t.Errorf("search_files unexpected matches: %v", matches)
		}
	}

	// Test invalid tool
	_, err = mcp.CallTool(ctx, "invalid_tool", nil)
	if err == nil {
		t.Error("Expected error for invalid tool, got nil")
	}

	// Verify ListTools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_CloudMode(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_cloud_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	t.Setenv("OHC_STANDALONE", "false")

	mcp := NewHybridFSMCP(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-cloud",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write_file tool
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "cloud_mcp.txt",
		"content": "cloud mcp content",
	})
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["status"] != "success" {
		t.Errorf("write_file unexpected result: %v", res)
	}

	onDiskPath := filepath.Join(tmpDir, "tenant-cloud", "cloud_mcp.txt")
	if _, err := os.Stat(onDiskPath); os.IsNotExist(err) {
		t.Errorf("File was not written to the correct tenant directory: %s", onDiskPath)
	}
}

func TestDirectoryTraversalExploit(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_exploit_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create tenant-1 directory with a secret file
	tenant1Dir := filepath.Join(tmpDir, "tenant-1")
	if err := os.MkdirAll(tenant1Dir, 0755); err != nil {
		t.Fatal(err)
	}
	secretPath := filepath.Join(tenant1Dir, "secret.txt")
	if err := os.WriteFile(secretPath, []byte("tenant-1 secret"), 0644); err != nil {
		t.Fatal(err)
	}

	// Create tenant-10 directory (the attacker)
	tenant10Dir := filepath.Join(tmpDir, "tenant-10")
	if err := os.MkdirAll(tenant10Dir, 0755); err != nil {
		t.Fatal(err)
	}

	// Simulate attacker (tenant-10) trying to read from tenant-1
	claims := &auth.Claims{
		OrganizationID: "tenant-10",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// The attacker provides a path that traverses out of tenant-10 and into tenant-1
	// The path becomes: /tmp/cloudfs_exploit_test/tenant-10/../tenant-1/secret.txt
	// After filepath.Clean, it becomes: /tmp/cloudfs_exploit_test/tenant-1/secret.txt
	_, err = provider.ReadFile(ctx, "../tenant-1/secret.txt")
	if err == nil {
		t.Error("Exploit successful: tenant-10 was able to read tenant-1's file using directory traversal!")
	} else if err.Error() != "path escapes tenant directory" {
		t.Errorf("Expected 'path escapes tenant directory', got: %v", err)
	}
}
