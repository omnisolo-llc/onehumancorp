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
	provider := &LocalFSProvider{WorkspaceRoot: tempDir}
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("Expected 'hello', got %q", string(content))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Directory traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("Expected error for directory traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tempDir}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-1"})

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("tenant-data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure file is in tenant dir
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); err != nil {
		t.Errorf("File not found in tenant directory: %v", err)
	}

	// Read file
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "tenant-data" {
		t.Errorf("Expected 'tenant-data', got %q", string(content))
	}

	// Missing claims
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err == nil {
		t.Error("Expected error for missing claims, got nil")
	}
}

func TestServer(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("WORKSPACE_ROOT", tempDir)
	server := NewServer()
	ctx := context.Background()

	// ListTools
	tools, err := server.ListTools(ctx)
	if err != nil {
		t.Fatalf("ListTools failed: %v", err)
	}
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Write
	writeArgs := []byte(`{"path": "foo.txt", "content": "bar"}`)
	res, err := server.CallTool(ctx, "write_file", writeArgs)
	if err != nil || res.IsError {
		t.Fatalf("Write failed: %v", err)
	}

	// Read
	readArgs := []byte(`{"path": "foo.txt"}`)
	res, err = server.CallTool(ctx, "read_file", readArgs)
	if err != nil || res.IsError {
		t.Fatalf("Read failed: %v", err)
	}
	if len(res.Content) == 0 || res.Content[0].Text != "bar" {
		t.Errorf("Expected content 'bar', got %v", res.Content)
	}
}

func TestLocalFSProviderSecurity(t *testing.T) {
	tempDir := t.TempDir()
	// Create a sibling directory to test traversal against
	siblingDir := filepath.Join(filepath.Dir(tempDir), "sibling")
	os.MkdirAll(siblingDir, 0755)
	defer os.RemoveAll(siblingDir)

	provider := &LocalFSProvider{WorkspaceRoot: tempDir}
	ctx := context.Background()

	// Test trying to read a sibling directory which starts with the same prefix string
	// but is not a subdirectory
	_, err := provider.ReadFile(ctx, "../sibling/test.txt")
	if err == nil {
		t.Error("Expected error when accessing sibling directory via ../")
	}
}

func TestCloudFSProviderSecurity(t *testing.T) {
	tempDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tempDir}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-1"})

	// Ensure tenant-1 and tenant-2 directories exist
	os.MkdirAll(filepath.Join(tempDir, "tenant-1"), 0755)
	os.MkdirAll(filepath.Join(tempDir, "tenant-2"), 0755)
	os.WriteFile(filepath.Join(tempDir, "tenant-2", "secret.txt"), []byte("secret"), 0644)

	// Try directory traversal to read from tenant-2
	_, err := provider.ReadFile(ctx, "../tenant-2/secret.txt")
	if err == nil {
		t.Error("Expected error when attempting directory traversal out of tenant sandbox, got nil")
	}
}
