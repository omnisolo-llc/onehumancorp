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
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	if !provider.IsLocal() {
		t.Errorf("expected local provider")
	}

	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("unexpected error writing file: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", string(data))
	}

	// Test list
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("unexpected error listing dir: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("unexpected list dir result: %v", infos)
	}

	// Test search
	matches, err := provider.SearchFiles(ctx, ".", "test.txt")
	if err != nil {
		t.Errorf("unexpected error searching files: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("unexpected search result: %v", matches)
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacked"))
	if err == nil {
		t.Errorf("expected error for path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	if provider.IsLocal() {
		t.Errorf("expected cloud provider")
	}

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test missing tenant
	emptyCtx := context.Background()
	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil {
		t.Errorf("expected error for missing tenant")
	}

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("unexpected error writing file: %v", err)
	}

	// Ensure it was written to tenant dir
	content, _ := os.ReadFile(filepath.Join(tmpDir, "tenant1", "test.txt"))
	if string(content) != "hello cloud" {
		t.Errorf("expected 'hello cloud' in tenant dir, got '%s'", string(content))
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	// Test list
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("unexpected error listing dir: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("unexpected list dir result: %v", infos)
	}

	// Test search
	matches, err := provider.SearchFiles(ctx, ".", "test.txt")
	if err != nil {
		t.Errorf("unexpected error searching files: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("unexpected search result: %v", matches)
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../tenant2/escape.txt", []byte("hacked"))
	if err == nil {
		t.Errorf("expected error for path traversal")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcpfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Write
	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Errorf("unexpected error calling write_file: %v", err)
	}
	writeMap := writeRes.(map[string]interface{})
	if writeMap["status"] != "success" {
		t.Errorf("expected success status")
	}

	// Read
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Errorf("unexpected error calling read_file: %v", err)
	}
	readMap := readRes.(map[string]interface{})
	if readMap["content"] != "mcp content" {
		t.Errorf("expected 'mcp content'")
	}

	// List
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Errorf("unexpected error calling list_directory: %v", err)
	}
	listMap := listRes.(map[string]interface{})
	files := listMap["files"].([]map[string]interface{})
	if len(files) != 1 || files[0]["name"] != "mcp.txt" {
		t.Errorf("unexpected list result")
	}

	// Search
	searchRes, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"dir":     ".",
		"pattern": "mcp.txt",
	})
	if err != nil {
		t.Errorf("unexpected error calling search_files: %v", err)
	}
	searchMap := searchRes.(map[string]interface{})
	matches := searchMap["matches"].([]string)
	if len(matches) != 1 || matches[0] != "mcp.txt" {
		t.Errorf("unexpected search result")
	}

	// Unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}
}

func TestFSFactory(t *testing.T) {
	// Standalone (default)
	os.Setenv("OHC_MULTITENANT", "")
	p1 := FSFactory()
	if !p1.IsLocal() {
		t.Errorf("expected local provider when OHC_MULTITENANT is unset")
	}

	// Cloud
	os.Setenv("OHC_MULTITENANT", "true")
	p2 := FSFactory()
	if p2.IsLocal() {
		t.Errorf("expected cloud provider when OHC_MULTITENANT=true")
	}
}
