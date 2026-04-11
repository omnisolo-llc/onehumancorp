package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_Bounds(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "fs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Valid path
	validPath := "test.txt"
	err = provider.WriteFile(ctx, validPath, []byte("hello"))
	if err != nil {
		t.Errorf("Unexpected error writing valid path: %v", err)
	}

	content, err := provider.ReadFile(ctx, validPath)
	if err != nil {
		t.Errorf("Unexpected error reading valid path: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("Expected 'hello', got %s", string(content))
	}

	// Boundary violation
	invalidPath := "../test.txt"
	err = provider.WriteFile(ctx, invalidPath, []byte("hello"))
	if err == nil {
		t.Errorf("Expected error writing out-of-bounds path, got nil")
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "fs_test_cloud")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	localProvider := NewLocalFSProvider(tempDir)
	cloudProvider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = cloudProvider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Verify it was written to the tenant subfolder using the base provider
	content, err := localProvider.ReadFile(context.Background(), filepath.Join("tenant-123", "data.txt"))
	if err != nil {
		t.Errorf("Failed to read file from local provider: %v", err)
	}
	if string(content) != "tenant data" {
		t.Errorf("Expected 'tenant data', got %s", string(content))
	}

	// Missing tenant ID should fail
	badCtx := context.Background()
	err = cloudProvider.WriteFile(badCtx, "data.txt", []byte("data"))
	if err == nil {
		t.Errorf("Expected error for missing tenant ID, got nil")
	}

	// Test ListDir
	files, err := cloudProvider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir error: %v", err)
	}
	if len(files) != 1 || files[0].Name != "data.txt" {
		t.Errorf("Expected 1 file 'data.txt', got %v", files)
	}

	// Test Search
	err = cloudProvider.WriteFile(ctx, "sub/another.txt", []byte("data"))
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	matches, err := cloudProvider.SearchFiles(ctx, "data")
	if err != nil {
		t.Errorf("SearchFiles error: %v", err)
	}
	if len(matches) != 1 {
		t.Errorf("Expected 1 match, got %v", matches)
	}

	// Test Path Traversal
	err = cloudProvider.WriteFile(ctx, "../other-tenant/secrets.txt", []byte("hack"))
	if err == nil {
		t.Errorf("Expected error for cross-tenant path traversal, got nil")
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	p1 := NewFileSystemProvider("/tmp")
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider in standalone mode")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p2 := NewFileSystemProvider("/tmp")
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider in cloud mode")
	}
}
