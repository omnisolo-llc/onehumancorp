package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	root := t.TempDir()
	os.Setenv("OHC_FS_ROOT", root)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := &LocalFSProvider{}
	ctx := context.Background()
	claims := &auth.Claims{}

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Fatalf("Expected hello, got %s", content)
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("Unexpected entries: %v", entries)
	}

	res, err := provider.SearchFiles(ctx, claims, ".", "test")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(res) != 1 {
		t.Fatalf("Unexpected search results: %v", res)
	}

	// Path traversal test
	_, err = provider.ReadFile(ctx, claims, "../outside.txt")
	if err == nil {
		t.Fatalf("Expected path traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	root := t.TempDir()
	os.Setenv("OHC_FS_ROOT", root)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := &CloudFSProvider{}
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Fatalf("Expected hello, got %s", content)
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("Unexpected entries: %v", entries)
	}

	// Path traversal test
	_, err = provider.ReadFile(ctx, claims, "../outside.txt")
	if err == nil {
		t.Fatalf("Expected path traversal error")
	}

	// Test unauthorized
	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err == nil {
		t.Fatalf("Expected unauthorized error")
	}
}

func TestHybridFSMCP(t *testing.T) {
	root := t.TempDir()
	os.Setenv("OHC_FS_ROOT", root)
	defer os.Unsetenv("OHC_FS_ROOT")

	mcp := NewHybridFSMCP(false) // local mode
	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello" {
		t.Fatalf("Expected hello, got %v", resMap["content"])
	}
}
