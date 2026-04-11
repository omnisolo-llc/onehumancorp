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
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test write
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got %s", string(data))
	}

	// Test list
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("unexpected entries: %v", entries)
	}

	// Test escape
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Error("expected error when path escapes base dir")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	// Test write
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Verify it wrote to the tenant dir
	b, err := os.ReadFile(filepath.Join(tempDir, "tenant1", "test.txt"))
	if err != nil || string(b) != "hello cloud" {
		t.Errorf("file not written to correct tenant dir: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(data))
	}

	// Test missing claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("fail"))
	if err == nil {
		t.Error("expected error when missing claims")
	}

	// Test escape
	err = provider.WriteFile(ctx, claims, "../outside.txt", []byte("fail"))
	if err == nil {
		t.Error("expected error when path escapes tenant dir")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Write
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "hello mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello mcp" {
		t.Errorf("expected 'hello mcp', got %v", resMap["content"])
	}
	if resMap["mode"] != "standalone" {
		t.Errorf("expected mode standalone, got %v", resMap["mode"])
	}

	// List
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listMap := resList.(map[string]interface{})
	results := listMap["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "mcp.txt" {
		t.Errorf("unexpected list results: %v", results)
	}
}

func TestListTools(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider("."))
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}

func TestCloudFSProvider_ListDir(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	if provider.IsLocal() {
		t.Error("expected IsLocal to be false")
	}

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("unexpected entries: %v", entries)
	}
}

func TestCallToolErrors(t *testing.T) {
	mcp := NewHybridFSMCP(NewLocalFSProvider("."))
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test"})
	if err == nil {
		t.Error("expected error for missing content")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"content": "test"})
	if err == nil {
		t.Error("expected error for missing path")
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path")
	}

	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for unknown tool")
	}
}

func TestInitProvider(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	p := InitProvider(".")
	if !p.IsLocal() {
		t.Error("expected local provider")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p = InitProvider(".")
	if p.IsLocal() {
		t.Error("expected cloud provider")
	}
}

func TestMissingClaimsCloud(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Error("expected error for missing claims in cloud mode")
	}

	_, err = provider.resolvePath(nil, "test.txt")
	if err == nil {
		t.Error("expected error for missing claims in cloud mode")
	}

	claims := &auth.Claims{OrganizationID: ""}
	_, err = provider.resolvePath(claims, "test.txt")
	if err == nil {
		t.Error("expected error for missing organization ID in claims")
	}
}

func TestWriteFileMkdirError(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()
	// create a file where a directory would be needed
	err := os.WriteFile(filepath.Join(tempDir, "file_not_dir"), []byte("file"), 0644)
	if err != nil {
		t.Fatal(err)
	}

	err = provider.WriteFile(ctx, nil, "file_not_dir/test.txt", []byte("test"))
	if err == nil {
		t.Error("expected error when mkdir fails")
	}

	cloudProvider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "file_not_dir"}
	err = cloudProvider.WriteFile(ctx, claims, "test.txt", []byte("test"))
	if err == nil {
		t.Error("expected error when mkdir fails in cloud mode")
	}
}

func TestMcpWrapperErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "../outside"})
	if err == nil {
		t.Error("expected error")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "../outside", "content": "test"})
	if err == nil {
		t.Error("expected error")
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "../outside"})
	if err == nil {
		t.Error("expected error")
	}
}

func TestResolvePathErrorsLocal(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, nil, "../outside.txt", []byte("test"))
	if err == nil {
		t.Error("expected error")
	}

	_, err = provider.ListDir(ctx, nil, "../outside")
	if err == nil {
		t.Error("expected error")
	}
}

func TestResolvePathErrorsCloud(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	_, err := provider.ReadFile(ctx, claims, "../outside.txt")
	if err == nil {
		t.Error("expected error")
	}

	_, err = provider.ListDir(ctx, claims, "../outside")
	if err == nil {
		t.Error("expected error")
	}
}
