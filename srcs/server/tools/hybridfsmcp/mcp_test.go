package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Errorf("unexpected directory entries: %+v", entries)
	}

	// Escape path
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Error("expected error for escaping path, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Errorf("unexpected directory entries: %+v", entries)
	}

	// Missing claims
	errCtx := context.Background()
	_, err = provider.ReadFile(errCtx, "test.txt")
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Escape path
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Error("expected error for escaping path, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// List tools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Call write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp data",
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	// Call read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["content"] != "mcp data" {
		t.Errorf("unexpected read_file result: %+v", res)
	}

	// Call list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected list_directory result type: %T", res)
	}
	entries, ok := resMap["entries"].([]map[string]interface{})
	if !ok || len(entries) != 1 || entries[0]["name"] != "mcp.txt" {
		t.Errorf("unexpected list_directory result: %+v", res)
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	p1 := NewFileSystemProvider("")
	if !p1.IsLocal() {
		t.Error("expected local provider when OHC_STANDALONE=true")
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2 := NewFileSystemProvider("")
	if p2.IsLocal() {
		t.Error("expected cloud provider when OHC_STANDALONE=false")
	}
}

func TestHybridFSMCP_SearchFiles(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	provider.WriteFile(ctx, "apple.txt", []byte("1"))
	provider.WriteFile(ctx, "banana.txt", []byte("2"))

	res, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":  ".",
		"query": "app",
	})
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	resMap, _ := res.(map[string]interface{})
	entries, _ := resMap["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "apple.txt" {
		t.Errorf("unexpected search_files result: %+v", res)
	}
}
