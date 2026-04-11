package mcp

import (
	"context"
	"os"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	testData := []byte("hello world")
	err = provider.WriteFile(ctx, "test.txt", testData)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != string(testData) {
		t.Errorf("Expected %s, got %s", testData, data)
	}

	// Test ListDir
	err = provider.WriteFile(ctx, "test2.txt", testData)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	expectedFiles := []string{"test.txt", "test2.txt"}
	if !reflect.DeepEqual(files, expectedFiles) {
		t.Errorf("Expected files %v, got %v", expectedFiles, files)
	}

	// Test Security: Path Escaping
	err = provider.WriteFile(ctx, "../escape.txt", testData)
	if err == nil {
		t.Errorf("Expected error for path escape, got nil")
	}
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Errorf("Expected error for path escape, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testData := []byte("cloud hello")
	err = provider.WriteFile(ctx, "data.txt", testData)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != string(testData) {
		t.Errorf("Expected %s, got %s", testData, data)
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "data.txt" {
		t.Errorf("Expected ['data.txt'], got %v", files)
	}

	// Test Security: Missing Claims
	ctxNoClaims := context.Background()
	err = provider.WriteFile(ctxNoClaims, "data.txt", testData)
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}

	// Test Security: Path Escaping Tenant Dir
	err = provider.WriteFile(ctx, "../other-tenant/data.txt", testData)
	if err == nil {
		t.Errorf("Expected error for tenant path escape, got nil")
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	// Test Cloud Mode
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_FS_ROOT", "/tmp/cloud")
	defer os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewFileSystemProvider()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", provider)
	}

	// Test Standalone Mode
	os.Setenv("OHC_MULTITENANT", "false")
	os.Setenv("OHC_FS_ROOT", "/tmp/local")
	provider = NewFileSystemProvider()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", provider)
	}
}
