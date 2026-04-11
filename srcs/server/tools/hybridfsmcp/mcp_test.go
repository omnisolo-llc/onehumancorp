package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "localfstest")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider, err := NewLocalFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("Expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Fatalf("Expected 1 file named 'test.txt', got %d", len(infos))
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for path traversal, got nil")
	}

	// Test Absolute Path
	err = provider.WriteFile(ctx, "/tmp/abs.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for absolute path, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfstest")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider, err := NewCloudFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	// Test context without claims
	ctx := context.Background()
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatalf("Expected error for missing org ID, got nil")
	}

	// Test context with claims
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})

	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test Tenant Isolation
	ctxOtherTenant := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant2",
	})

	_, err = provider.ReadFile(ctxOtherTenant, "test.txt")
	if err == nil {
		t.Fatalf("Expected error when reading other tenant's file")
	}

	// Test Path Traversal
	err = provider.WriteFile(ctxWithClaims, "../tenant2/test.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected error for path traversal, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	dir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider, err := NewLocalFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	// WriteFile
	resWrite, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if resWrite.(map[string]interface{})["status"] != "success" {
		t.Fatalf("Expected success")
	}

	// ReadFile
	resRead, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if resRead.(map[string]interface{})["content"] != "world" {
		t.Fatalf("Expected 'world', got '%v'", resRead.(map[string]interface{})["content"])
	}

	// ListDir
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}
	results := resList.(map[string]interface{})["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "hello.txt" {
		t.Fatalf("Expected 1 file 'hello.txt', got %v", results)
	}

	// Cloud MCP Authorization
	cloudProvider, err := NewCloudFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create cloud provider: %v", err)
	}
	cloudMcp := NewHybridFSMCP(cloudProvider)

	_, err = cloudMcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Fatalf("Expected auth error in cloud mode without context")
	}

	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})
	_, err = cloudMcp.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "test",
	})
	if err != nil {
		t.Fatalf("write_file with claims failed: %v", err)
	}
}
