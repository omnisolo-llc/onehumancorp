package fsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test write and read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", entries)
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("expected error on traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write and read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%s'", string(data))
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tempDir, "tenant1", "test.txt")); os.IsNotExist(err) {
		t.Fatalf("expected file to be created in tenant dir")
	}

	// Test with no claims
	ctxNoClaims := context.Background()
	err = provider.WriteFile(ctxNoClaims, "test.txt", []byte("fail"))
	if err == nil {
		t.Fatalf("expected error without claims")
	}
}

func TestFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	// List tools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Write file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp content" {
		t.Fatalf("expected 'mcp content', got '%v'", resMap["content"])
	}

	// List dir
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	listResMap := listRes.(map[string]interface{})
	entries := listResMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp.txt" {
		t.Fatalf("expected ['mcp.txt'], got %v", entries)
	}
}

func TestNewProviderFactory(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	provider := NewProviderFactory("/tmp")
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Fatalf("expected CloudFSProvider")
	}

	t.Setenv("OHC_MULTITENANT", "")
	provider = NewProviderFactory("/tmp")
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider")
	}
}
