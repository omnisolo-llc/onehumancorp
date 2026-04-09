package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fs_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := mcp.NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Should block traversal
	_, err = provider.ReadFile(ctx, "../some_file.txt")
	if err != mcp.ErrPathTraversal {
		t.Fatalf("expected ErrPathTraversal, got %v", err)
	}

	_, err = provider.ReadFile(ctx, "/etc/passwd")
	// Clean will convert /etc/passwd -> etc/passwd. rel = etc/passwd
	// We want to make sure it doesn't read the real /etc/passwd
	if err == nil {
		t.Fatal("expected error reading non-existent local file, got nil")
	}

	// Should allow write and read inside
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_fs_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := mcp.NewCloudFSProvider(tmpDir)

	// Context without claims should fail
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err != mcp.ErrUnauthorized {
		t.Fatalf("expected ErrUnauthorized, got %v", err)
	}

	// Valid context
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Path traversal block
	_, err = provider.ReadFile(ctx, "../tenant2/test.txt")
	if err != mcp.ErrPathTraversal {
		t.Fatalf("expected ErrPathTraversal, got %v", err)
	}

	// Write and read
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant1_data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "tenant1_data" {
		t.Fatalf("expected 'tenant1_data', got '%s'", string(data))
	}

	// Verify physical location is scoped
	physicalData, err := os.ReadFile(filepath.Join(tmpDir, "tenant1", "data.txt"))
	if err != nil {
		t.Fatalf("could not read physical file: %v", err)
	}
	if string(physicalData) != "tenant1_data" {
		t.Fatalf("physical file content mismatch")
	}
}

func TestHybridFSMCP_Tools(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybrid_mcp_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	// Test Standalone mode (LocalFS)
	h := NewHybridFSMCP(true, tmpDir)
	ctx := context.Background()

	// ListTools
	tools := h.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// CallTool write_file
	_, err = h.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	// CallTool read_file
	res, err := h.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if resMap["content"] != "world" {
		t.Fatalf("expected content 'world', got %v", resMap["content"])
	}

	// CallTool list_directory
	res, err = h.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	entries, ok := resMap["entries"].([]map[string]interface{})
	if !ok || len(entries) != 1 {
		t.Fatalf("expected 1 entry in list, got %v", resMap["entries"])
	}
	if entries[0]["name"] != "hello.txt" {
		t.Fatalf("expected file 'hello.txt', got %v", entries[0]["name"])
	}
}
