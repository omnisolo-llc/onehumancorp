package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatal(err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got %q", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %v", infos)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Errorf("expected error on path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false")
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	// Test WriteFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatal(err)
	}

	// Verify tenant isolation physically
	tenantPath := filepath.Join(tempDir, "tenant1", "test.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Errorf("expected file to exist at %s", tenantPath)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %q", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %v", infos)
	}

	// Test Path Traversal and Tenant Isolation
	_, err = provider.ReadFile(ctx, claims, "../tenant2/test.txt")
	if err == nil {
		t.Errorf("expected error on path traversal/tenant isolation violation")
	}

	// Test missing claims
	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err == nil {
		t.Errorf("expected error with missing claims")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test Write
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatal(err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected write result: %v", resMap)
	}

	// Test Read
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatal(err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["content"] != "mcp content" {
		t.Errorf("unexpected read result: %v", resMap)
	}

	// Test List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatal(err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "mcp.txt" {
		t.Errorf("unexpected list result: %v", resMap)
	}

	// Test Search
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path": ".",
		"term": "mcp",
	})
	if err != nil {
		t.Fatal(err)
	}
	resMap = res.(map[string]interface{})
	matches := resMap["matches"].([]string)
	if len(matches) != 1 || matches[0] != "mcp.txt" {
		t.Errorf("unexpected search result: %v", resMap)
	}
}
