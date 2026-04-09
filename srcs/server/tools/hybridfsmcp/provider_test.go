package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Valid write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("failed to write valid file: %v", err)
	}

	// Read valid file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Errorf("failed to read valid file or content mismatch: %v", err)
	}

	// Path traversal attempt
	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for path traversal, got nil")
	}

	// Absolute path attempt
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for absolute path, got nil")
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Valid write
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Errorf("failed to write file for tenant: %v", err)
	}

	// Verify it wrote to the right scoped directory
	expectedPath := filepath.Join(tempDir, "tenant-1", "data.txt")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("file was not created at expected tenant path: %s", expectedPath)
	}

	// Path traversal attempt within tenant
	err = provider.WriteFile(ctx, "../../other-tenant/data.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for path traversal escaping tenant dir, got nil")
	}

	// Absolute path attempt
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for absolute path, got nil")
	}

	// Test missing claims
	err = provider.WriteFile(context.Background(), "data.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error when no claims in context, got nil")
	}
}
