package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	workspace := t.TempDir()
	provider := NewLocalFSProvider(workspace)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(data))
	}

	// Test path escape standard
	err = provider.WriteFile(ctx, claims, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}

	// Test prefix sharing attack
	wsBase := filepath.Base(workspace)
	wsParent := filepath.Dir(workspace)
	err = provider.WriteFile(ctx, claims, "../" + wsBase + "-sibling/escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error for prefix sharing attack, workspace base is %s, parent is %s", wsBase, wsParent)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Errorf("Expected 1 entry 'test.txt', got %v", entries)
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseDir := t.TempDir()
	provider := NewCloudFSProvider(baseDir)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test tenant isolation
	if _, err := os.Stat(filepath.Join(baseDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Error("File was not written to tenant directory")
	}

	// Test path escape standard
	err = provider.WriteFile(ctx, claims, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}

	// Test prefix sharing attack
	err = provider.WriteFile(ctx, claims, "../tenant-10/escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error for prefix sharing attack, got nil")
	}

	// Test missing claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error for missing claims, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	workspace := t.TempDir()
	provider := NewLocalFSProvider(workspace)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Call write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "mcp test",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Call read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	m := res.(map[string]interface{})
	if m["content"] != "mcp test" {
		t.Errorf("Expected 'mcp test', got '%v'", m["content"])
	}

	// Call list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	m = res.(map[string]interface{})
	entries := m["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "hello.txt" {
		t.Errorf("Expected 1 entry 'hello.txt', got %v", entries)
	}

	// Verify search_files is removed from list
	tools := mcp.ListTools()
	for _, tool := range tools {
		if tool.Name == "search_files" {
			t.Errorf("search_files should not be in tools list")
		}
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	p := NewProvider()
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Error("Expected LocalFSProvider in standalone mode")
	}

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	p = NewProvider()
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Error("Expected CloudFSProvider in multitenant mode")
	}
}
