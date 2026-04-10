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
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test Directory Traversal Prevention
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected error for traversal, got none")
	}

	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("expected error for absolute path, got none")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create context with claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(content) != "tenant data" {
		t.Errorf("expected 'tenant data', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "data.txt" {
		t.Errorf("expected ['data.txt'], got %v", entries)
	}

	// Test Directory Traversal Prevention
	_, err = provider.ReadFile(ctx, "../tenant2/data.txt")
	if err == nil {
		t.Errorf("expected error for traversal, got none")
	}

	// Test Tenant Directory Creation
	tenantDir := filepath.Join(tmpDir, "tenant1")
	if _, err := os.Stat(tenantDir); os.IsNotExist(err) {
		t.Errorf("expected tenant directory to be created, but it was not")
	}

	// Test without claims
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "data.txt")
	if err == nil {
		t.Errorf("expected error for missing claims, got none")
	}
}

func TestFSInspectorMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewFSInspectorMCP(provider)
	ctx := context.Background()

	// Test ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Test CallTool - write_file
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp data",
	}
	res, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if resMap["status"] != "success" {
			t.Errorf("expected success status, got %v", resMap["status"])
		}
	} else {
		t.Errorf("unexpected response type: %T", res)
	}

	// Test CallTool - read_file
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	res, err = mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if resMap["status"] != "success" {
			t.Errorf("expected success status, got %v", resMap["status"])
		}
		if resMap["content"] != "mcp data" {
			t.Errorf("expected 'mcp data', got %v", resMap["content"])
		}
	} else {
		t.Errorf("unexpected response type: %T", res)
	}

	// Test CallTool - list_directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if resMap["status"] != "success" {
			t.Errorf("expected success status, got %v", resMap["status"])
		}
		if entries, ok := resMap["entries"].([]string); ok {
			if len(entries) != 1 || entries[0] != "mcp_test.txt" {
				t.Errorf("expected ['mcp_test.txt'], got %v", entries)
			}
		} else {
			t.Errorf("unexpected entries type: %T", resMap["entries"])
		}
	} else {
		t.Errorf("unexpected response type: %T", res)
	}

	// Test Factory
	local := FSProviderFactory(true, tmpDir)
	if _, ok := local.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider")
	}

	cloud := FSProviderFactory(false, tmpDir)
	if _, ok := cloud.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider")
	}
}
