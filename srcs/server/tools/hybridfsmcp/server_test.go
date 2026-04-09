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
	provider := &LocalFSProvider{BaseDir: tmpDir}
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
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected ListDir result: %v", entries)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tmpDir}

	// Unauthenticated
	ctxUnauth := context.Background()
	_, err := provider.ReadFile(ctxUnauth, "test.txt")
	if err == nil {
		t.Error("Expected error for unauthenticated ReadFile, got nil")
	}

	// Authenticated
	claims := &auth.Claims{OrganizationID: "org1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tmpDir, "org1", "test.txt")); os.IsNotExist(err) {
		t.Error("File was not written to tenant directory")
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected ListDir result: %v", entries)
	}
}

func TestServerCallTool(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewServer(provider)
	ctx := context.Background()

	// WriteFile
	_, err := server.CallTool(ctx, ToolWriteFile, map[string]interface{}{
		"path":    "server_test.txt",
		"content": "server content",
	})
	if err != nil {
		t.Fatalf("CallTool(WriteFile) failed: %v", err)
	}

	// ReadFile
	res, err := server.CallTool(ctx, ToolReadFile, map[string]interface{}{
		"path": "server_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool(ReadFile) failed: %v", err)
	}
	if res.(string) != "server content" {
		t.Errorf("Expected 'server content', got '%v'", res)
	}

	// ListDir
	res, err = server.CallTool(ctx, ToolListDir, map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool(ListDir) failed: %v", err)
	}
	names := res.([]string)
	if len(names) != 1 || names[0] != "server_test.txt" {
		t.Errorf("Unexpected ListDir result: %v", names)
	}

	// Unknown tool
	_, err = server.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Error("Expected error for unknown tool, got nil")
	}
}

func TestNewProvider(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	p := NewProvider()
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider when OHC_MULTITENANT is true")
	}

	t.Setenv("OHC_MULTITENANT", "false")
	p = NewProvider()
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider when OHC_MULTITENANT is false")
	}
}
