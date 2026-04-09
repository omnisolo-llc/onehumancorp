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
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"), nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	content, err := provider.ReadFile(ctx, "test.txt", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got %s", string(content))
	}

	// List directory
	entries, err := provider.ListDir(ctx, ".", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("expected 1 entry 'test.txt', got %v", entries)
	}

	// Path traversal attempt
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"), nil)
	if err == nil || err.Error() != "path traversal denied" {
		t.Errorf("expected path traversal denied, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Without claims
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"), nil)
	if err == nil || err.Error() != "unauthorized: missing claims or organization ID" {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	// With claims, write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"), claims)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Check underlying path
	contentBytes, err := os.ReadFile(filepath.Join(tmpDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("failed to read actual file: %v", err)
	}
	if string(contentBytes) != "hello" {
		t.Errorf("expected 'hello' in actual file, got %s", string(contentBytes))
	}

	// Read file
	content, err := provider.ReadFile(ctx, "test.txt", claims)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got %s", string(content))
	}

	// List directory
	entries, err := provider.ListDir(ctx, ".", claims)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("expected 1 entry 'test.txt', got %v", entries)
	}

	// Path traversal attempt
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"), claims)
	if err == nil || err.Error() != "path traversal denied" {
		t.Errorf("expected path traversal denied, got %v", err)
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// Call tool write_file
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "mcp hello",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", resMap["status"])
	}

	// Call tool read_file
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["content"] != "mcp hello" {
		t.Errorf("expected 'mcp hello', got %v", resMap["content"])
	}

	// Call tool list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected test.txt, got %v", entries)
	}
}
