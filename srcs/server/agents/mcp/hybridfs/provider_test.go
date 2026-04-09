package hybridfs

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("unexpected error writing file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("unexpected error listing directory: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../outside.txt", []byte("hacked"))
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got %v", err)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Errorf("unexpected error writing file: %v", err)
	}

	// Verify it wrote to the correct physical path
	physicalData, err := os.ReadFile(filepath.Join(tempDir, "tenant-123", "data.txt"))
	if err != nil {
		t.Errorf("failed to read physical file: %v", err)
	}
	if string(physicalData) != "tenant data" {
		t.Errorf("expected 'tenant data', got '%s'", string(physicalData))
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Errorf("unexpected error reading file: %v", err)
	}
	if string(data) != "tenant data" {
		t.Errorf("expected 'tenant data', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("unexpected error listing directory: %v", err)
	}
	if len(entries) != 1 || entries[0] != "data.txt" {
		t.Errorf("expected ['data.txt'], got %v", entries)
	}

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../outside.txt", []byte("hacked"))
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got %v", err)
	}

	// Test Cross-Tenant Access Protection
	err = provider.WriteFile(ctx, "../tenant-456/outside.txt", []byte("hacked"))
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for cross-tenant path traversal, got %v", err)
	}

	// Test Unauthorized (No Claims)
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "data.txt")
	if err != ErrUnauthorized {
		t.Errorf("expected ErrUnauthorized, got %v", err)
	}
}

func TestNewProvider(t *testing.T) {
	tempDir := t.TempDir()

	// Test Cloud Provider Factory
	cloudProvider, err := NewProvider(true, tempDir)
	if err != nil {
		t.Errorf("failed to create cloud provider: %v", err)
	}
	if _, ok := cloudProvider.(*CloudFSProvider); !ok {
		t.Errorf("expected *CloudFSProvider")
	}

	// Test Local Provider Factory
	localProvider, err := NewProvider(false, tempDir)
	if err != nil {
		t.Errorf("failed to create local provider: %v", err)
	}
	if _, ok := localProvider.(*LocalFSProvider); !ok {
		t.Errorf("expected *LocalFSProvider")
	}
}
