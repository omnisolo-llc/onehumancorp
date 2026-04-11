package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	// Test list
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Fatalf("Expected 1 file named 'test.txt', got %v", infos)
	}

	// Test path traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("evil"))
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}
	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	// Without claims, should fail
	ctx := context.Background()
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err == nil {
		t.Fatalf("Expected auth error, got nil")
	}

	// With claims, should succeed and be scoped to tenant
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Check underlying file structure
	_, err = os.Stat(filepath.Join(tmpDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("File not created in tenant directory: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	// Test cross-tenant access (should fail or write to own dir)
	claims2 := &auth.Claims{OrganizationID: "tenant-2"}
	ctxWithClaims2 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims2)

	_, err = provider.ReadFile(ctxWithClaims2, "test.txt")
	if err == nil {
		t.Fatalf("Expected error reading cross-tenant file, got nil")
	}

	// Test path traversal
	err = provider.WriteFile(ctxWithClaims, "../escape.txt", []byte("evil"))
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Write
	argsWrite := map[string]interface{}{
		"path": "test.txt",
		"data": "hello mcp",
	}
	_, err := mcp.CallTool(ctx, "write_file", argsWrite)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Read
	argsRead := map[string]interface{}{
		"path": "test.txt",
	}
	resRead, err := mcp.CallTool(ctx, "read_file", argsRead)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	readMap := resRead.(map[string]interface{})
	if readMap["data"] != "hello mcp" {
		t.Fatalf("Expected 'hello mcp', got '%v'", readMap["data"])
	}

	// List
	argsList := map[string]interface{}{
		"path": ".",
	}
	resList, err := mcp.CallTool(ctx, "list_directory", argsList)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listMap := resList.(map[string]interface{})
	results := listMap["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "test.txt" {
		t.Fatalf("Expected 1 file 'test.txt', got %v", results)
	}
}

func TestFactory(t *testing.T) {
	tmpDir := t.TempDir()

	t.Setenv("OHC_STANDALONE", "true")
	p1, err := NewProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider, got %T", p1)
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2, err := NewProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider, got %T", p2)
	}
}
