package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)

	// Test WriteFile
	ctx := context.Background()
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal Prevention
	err = provider.WriteFile(ctx, nil, "../outside.txt", []byte("hacker"))
	if err == nil {
		t.Errorf("Expected error when trying to write outside baseDir, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}

	ctx := context.Background()
	// Test WriteFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Verify it wrote to the correct tenant dir
	content, err := os.ReadFile(filepath.Join(tempDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("Failed to verify raw file: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(content))
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "test2.txt", []byte("fail"))
	if err == nil {
		t.Errorf("Expected error with missing claims, got nil")
	}

	// Test Path Traversal Prevention
	err = provider.WriteFile(ctx, claims, "../tenant-2/test.txt", []byte("hacker"))
	if err == nil {
		t.Errorf("Expected error when trying to write outside tenant directory, got nil")
	}

    // Test Cross Tenant
	claims2 := &auth.Claims{
		OrganizationID: "tenant-2",
	}
    _, err = provider.ReadFile(ctx, claims2, "../tenant-1/test.txt")
    if err == nil {
        t.Errorf("Expected error when trying to read other tenant directory via traversal, got nil")
    }
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Test write_file tool
	contentBase64 := base64.StdEncoding.EncodeToString([]byte("mcp tool test"))
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "tool_test.txt",
		"content": contentBase64,
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}

	// Test read_file tool
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "tool_test.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}
	if resMap["content"] != contentBase64 {
		t.Errorf("Expected %s, got %v", contentBase64, resMap["content"])
	}

	// Test list_directory tool
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "tool_test.txt" {
		t.Errorf("Expected ['tool_test.txt'], got %v", entries)
	}

	// Test fallback to string content decoding
	res, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "raw_test.txt",
		"content": "raw string content",
	})
	if err != nil {
		t.Fatalf("write_file raw failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", resMap["status"])
	}

	// Test ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
	    t.Errorf("Expected 3 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_CloudMultiTenantEnforcement(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-mt-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridFSMCP(NewCloudFSProvider(tempDir))
	ctx := context.Background()

	// Call without claims should fail early
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Errorf("Expected error for missing claims in multitenant mode")
	}

	// Call with claims should reach the provider
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// write_file
	_, err = mcp.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello",
	})
	if err != nil {
		t.Fatalf("Expected success with claims, got %v", err)
	}
}
