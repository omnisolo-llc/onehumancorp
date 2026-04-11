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

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got %q", string(content))
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0]["name"] != "test.txt" {
		t.Errorf("unexpected ListDir result: %v", entries)
	}

	// Test path traversal protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	localProvider, _ := NewLocalFSProvider(tempDir)
	cloudProvider := NewCloudFSProvider(localProvider)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write
	err := cloudProvider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to tenant-1/test.txt in the underlying provider
	content, err := os.ReadFile(filepath.Join(tempDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("failed to verify underlying file: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %q", string(content))
	}

	// Test read
	readContent, err := cloudProvider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %q", string(readContent))
	}

	// Test missing claims
	err = cloudProvider.WriteFile(context.Background(), "test.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Test write_file tool
	writeArgs := map[string]interface{}{
		"path":    "hello.txt",
		"content": "mcp testing",
	}
	_, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Test read_file tool
	readArgs := map[string]interface{}{
		"path": "hello.txt",
	}
	res, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["content"] != "mcp testing" {
		t.Errorf("unexpected read_file result: %v", res)
	}

	// Test list_directory tool
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok || len(resMap["results"].([]map[string]interface{})) != 1 {
		t.Errorf("unexpected list_directory result: %v", res)
	}
}
