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
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
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
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Error("expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-1"})

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify Tenant Isolation
	tenantDir := filepath.Join(tempDir, "tenant-1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Errorf("file not created in tenant dir: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test Missing Claims
	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Error("expected path traversal error, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test WriteFile
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "mcp.txt", "content": "mcp data"})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	if status := res.(map[string]interface{})["status"]; status != "success" {
		t.Errorf("expected status success, got %v", status)
	}

	// Test ReadFile
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "mcp.txt"})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	if content := res.(map[string]interface{})["content"]; content != "mcp data" {
		t.Errorf("expected 'mcp data', got %v", content)
	}

	// Test ListDir
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	entries := res.(map[string]interface{})["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp.txt" {
		t.Errorf("expected ['mcp.txt'], got %v", entries)
	}
}
