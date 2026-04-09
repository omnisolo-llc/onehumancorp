package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "local_fs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test writing and reading
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %q", string(data))
	}

	// Test directory traversal prevention
	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error writing outside workspace bounds")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	// Create a context with claims
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test writing
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Verify the file was written to the correct tenant directory
	expectedPath := filepath.Join(tempDir, "tenant-123", "data.txt")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("file was not written to the tenant directory at %s", expectedPath)
	}

	// Test reading
	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "tenant data" {
		t.Errorf("expected 'tenant data', got %q", string(data))
	}

	// Test missing context
	badCtx := context.Background()
	_, err = provider.ReadFile(badCtx, "data.txt")
	if err == nil {
		t.Error("expected error when reading without tenant context")
	}
}

func TestMCPFactoryAndTools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_factory_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_WORKSPACE_ROOT", tempDir)

	factory := NewMCPFactory()
	ctx := context.Background()

	// Test write_file tool
	res := factory.ExecuteTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	})
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Test read_file tool
	res = factory.ExecuteTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Test list_directory
	res = factory.ExecuteTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}
}
