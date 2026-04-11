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
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	testPath := "test_file.txt"
	testContent := []byte("hello world")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("Expected content %s, got %s", string(testContent), string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != testPath {
		t.Errorf("ListDir returned unexpected entries: %v", infos)
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// No claims context
	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}

	// With claims context
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testPath := "tenant_file.txt"
	testContent := []byte("tenant data")
	err = provider.WriteFile(ctxWithClaims, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to tenant dir
	tenantDir := filepath.Join(tempDir, "tenant-1")
	actualContent, err := os.ReadFile(filepath.Join(tenantDir, testPath))
	if err != nil {
		t.Fatalf("Failed to read actual file: %v", err)
	}
	if string(actualContent) != string(testContent) {
		t.Errorf("Expected content %s, got %s", string(testContent), string(actualContent))
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctxWithClaims, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("Expected content %s, got %s", string(testContent), string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctxWithClaims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != testPath {
		t.Errorf("ListDir returned unexpected entries: %v", infos)
	}

	// Test Path Traversal outside tenant dir
	err = provider.WriteFile(ctxWithClaims, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}
}

func TestFactory(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "factory_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_MULTITENANT", "true")
	p1, _ := NewProviderFactory(tempDir)
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2, _ := NewProviderFactory(tempDir)
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider")
	}

	os.Unsetenv("OHC_MULTITENANT")
}
