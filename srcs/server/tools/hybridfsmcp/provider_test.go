package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()

	// Write a file to test directory
	testFile := "test.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(context.Background(), testFile, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read the file back
	data, err := provider.ReadFile(context.Background(), testFile)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, data)
	}

	// List directory
	entries, err := provider.ListDir(context.Background(), ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != testFile {
		t.Errorf("ListDir unexpected result")
	}

	// Test path traversal attempt
	traversalPath := "../traversal.txt"
	err = provider.WriteFile(context.Background(), traversalPath, testContent)
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got: %v", err)
	}

	_, err = provider.ReadFile(context.Background(), traversalPath)
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got: %v", err)
	}

	_, err = provider.ListDir(context.Background(), "..")
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for path traversal, got: %v", err)
	}
}

func TestCloudFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewCloudFSProvider()
	orgID := "tenant-1"

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: orgID,
	})

	// Test writing
	testFile := "cloud.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, testFile, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure it created the tenant subdirectory
	expectedPath := filepath.Join(tempDir, orgID, testFile)
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("file not created in tenant subdirectory: %v", expectedPath)
	}

	// Test reading
	data, err := provider.ReadFile(ctx, testFile)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, data)
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != testFile {
		t.Errorf("ListDir unexpected result")
	}

	// Test path traversal attempt within same organization
	traversalPath := "../tenant-1/cloud.txt"
	data, err = provider.ReadFile(ctx, traversalPath)
	if err != nil {
		t.Errorf("valid traversal path failed: %v", err)
	}

	// Test path traversal outside organization
	invalidTraversal := "../tenant-2/cloud.txt"
	err = provider.WriteFile(ctx, invalidTraversal, testContent)
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for outside path traversal, got: %v", err)
	}

	// Test with no claims
	_, err = provider.ReadFile(context.Background(), testFile)
	if err == nil {
		t.Errorf("expected error without claims")
	}
}
