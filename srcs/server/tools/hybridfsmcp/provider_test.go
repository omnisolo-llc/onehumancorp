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
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got %s", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Error("Expected error on path traversal attempt")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	local := NewLocalFSProvider(tempDir)
	cloud := NewCloudFSProvider(local)

	// Create valid context
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write
	err := cloud.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify local path
	localPath := filepath.Join(tempDir, "tenant1", "test.txt")
	data, err := os.ReadFile(localPath)
	if err != nil {
		t.Fatalf("Failed to read actual file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test read
	data, err = cloud.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test ListDir
	entries, err := cloud.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal escaping the tenant dir
	err = cloud.WriteFile(ctx, "../tenant2/secret.txt", []byte("escape"))
	if err == nil {
		t.Error("Expected error on cross-tenant path traversal attempt")
	}

	// Test without tenant
	ctxNoAuth := context.Background()
	_, err = cloud.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Error("Expected error when no organization ID in context")
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	tempDir := t.TempDir()

	// Test standalone
	t.Setenv("OHC_MULTITENANT", "false")
	prov1 := NewFileSystemProvider(tempDir)
	if _, ok := prov1.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", prov1)
	}

	// Test cloud
	t.Setenv("OHC_MULTITENANT", "true")
	prov2 := NewFileSystemProvider(tempDir)
	if _, ok := prov2.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", prov2)
	}
}
