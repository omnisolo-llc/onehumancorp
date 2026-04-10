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

	if !provider.IsLocal() {
		t.Error("Expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("ListDir returned unexpected files: %v", files)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected matches: %v", matches)
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Error("Expected error for path escalation, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	if provider.IsLocal() {
		t.Error("Expected IsLocal to be false")
	}

	ctx := context.Background()

	// Test without claims
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("Expected missing claims error, got: %v", err)
	}

	// Test with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello tenant"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test physical isolation
	tenantFile := filepath.Join(tempDir, "tenant-1", "test.txt")
	data, err := os.ReadFile(tenantFile)
	if err != nil {
		t.Fatalf("Failed to read underlying file: %v", err)
	}
	if string(data) != "hello tenant" {
		t.Errorf("Expected 'hello tenant', got '%s'", string(data))
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != "hello tenant" {
		t.Errorf("Expected 'hello tenant', got '%s'", string(readData))
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../tenant-2/test.txt")
	if err == nil {
		t.Error("Expected error for path escalation, got nil")
	}
}

func TestHybridFSInspectorMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSInspectorMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test write_file
	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "mcp data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected success, got %v", writeRes)
	}

	// Test read_file
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["data"] != "mcp data" {
		t.Errorf("Expected 'mcp data', got %v", readRes)
	}

	// Test unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for unknown tool")
	}
}

func TestFactory(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	p1 := NewFileSystemProvider()
	if !p1.IsLocal() {
		t.Error("Expected local provider")
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2 := NewFileSystemProvider()
	if p2.IsLocal() {
		t.Error("Expected cloud provider")
	}
}
