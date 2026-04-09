package hybridfsmcp

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewFileSystemProvider(false, dir)

	ctx := context.Background()
	claims := &auth.Claims{} // Local doesn't need claims but passes them

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected hello, got %s", string(data))
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("expected 1 entry (test.txt), got %v", entries)
	}

	_, err = provider.ReadFile(ctx, claims, "../test.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal") {
		t.Errorf("expected path traversal error, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewFileSystemProvider(true, dir)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-A"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected hello, got %s", string(data))
	}

	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("expected 1 entry (test.txt), got %v", entries)
	}

	_, err = provider.ReadFile(ctx, claims, "../test.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal") {
		t.Errorf("expected path traversal error, got %v", err)
	}

   claimsB := &auth.Claims{OrganizationID: "tenant-B"}
   _, err = provider.ReadFile(ctx, claimsB, "test.txt")
	if err == nil {
		t.Errorf("expected error when reading other tenant's file")
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	dir := t.TempDir()
	provider := NewFileSystemProvider(false, dir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Write
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "Success" {
		t.Errorf("expected Success, got %v", res)
	}

	// Read
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "world" {
		t.Errorf("expected world, got %v", res)
	}

	// List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !strings.Contains(res.(string), "hello.txt") {
		t.Errorf("expected hello.txt in list, got %v", res)
	}

	// Tools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}
