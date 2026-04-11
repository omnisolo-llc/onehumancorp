package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("unexpected ListDir result: %+v", entries)
	}

	// Test Directory Traversal Prevention
	_, err = provider.ReadFile(ctx, nil, "../test.txt")
	if err == nil {
		t.Fatalf("expected error for path escaping base dir")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatalf("expected error for missing claims")
	}

	// Test WriteFile with claims
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Fatalf("file not created in tenant directory")
	}

	// Test ReadFile with claims
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir with claims
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("unexpected ListDir result: %+v", entries)
	}

	// Test Directory Traversal Prevention
	_, err = provider.ReadFile(ctx, claims, "../test.txt")
	if err == nil {
		t.Fatalf("expected error for path escaping tenant dir")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// CallTool write_file
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "world",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if status := res.(map[string]interface{})["status"]; status != "success" {
		t.Fatalf("expected success, got %v", status)
	}

	// CallTool read_file
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if data := res.(map[string]interface{})["data"]; data != "world" {
		t.Fatalf("expected 'world', got %v", data)
	}

	// CallTool list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := res.(map[string]interface{})["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "test.txt" {
		t.Fatalf("unexpected entries: %+v", entries)
	}
}
