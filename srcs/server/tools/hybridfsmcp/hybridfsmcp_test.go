package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create LocalFSProvider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacker"))
	if err == nil {
		t.Errorf("expected error for path traversal, got nil")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Errorf("expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create CloudFSProvider: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Verify it was written in the tenant directory
	tenantFile := filepath.Join(tempDir, "tenant1", "test.txt")
	if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
		t.Errorf("file not written to correct tenant directory: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacker"))
	if err == nil {
		t.Errorf("expected error for path traversal, got nil")
	}

	// Test No Organization ID
	emptyCtx := context.Background()
	err = provider.WriteFile(emptyCtx, "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error for missing organization ID, got nil")
	}
}
