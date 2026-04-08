package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathBounding(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test writing a file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Test reading the file
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(content))
	}

	// Test listing directory
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list directory: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacked"))
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test writing a file
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Verify the file was written to the correct tenant directory
	absPath := filepath.Join(tempDir, "tenant-123", "data.txt")
	if _, err := os.Stat(absPath); os.IsNotExist(err) {
		t.Errorf("file was not written to the tenant directory: %s", absPath)
	}

	// Test reading the file
	content, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(content) != "tenant data" {
		t.Errorf("expected 'tenant data', got '%s'", string(content))
	}

	// Test missing claims
	ctxNoClaims := context.Background()
	err = provider.WriteFile(ctxNoClaims, "data.txt", []byte("data"))
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacked"))
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestHybridFSServer_ToolRouting(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "hybridfs_server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	server, err := NewHybridFSServer(true, tempDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	ctx := context.Background()

	// ListTools
	tools := server.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// write_file
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	// read_file
	result, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resultMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map result, got %T", result)
	}
	if resultMap["content"] != "world" {
		t.Errorf("expected 'world', got '%v'", resultMap["content"])
	}

	// list_directory
	result, err = server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resultMap, ok = result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map result, got %T", result)
	}
	entries, ok := resultMap["entries"].([]string)
	if !ok {
		t.Fatalf("expected string slice, got %T", resultMap["entries"])
	}
	if len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("expected ['hello.txt'], got %v", entries)
	}
}
