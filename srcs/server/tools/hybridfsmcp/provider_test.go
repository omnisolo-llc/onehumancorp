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

	provider := NewFileSystemProvider(true, tempDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, nil, "subdir/test2.txt", []byte("hello again"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, nil, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}

	foundTest := false
	foundSubdir := false
	for _, entry := range entries {
		if entry == "test.txt" {
			foundTest = true
		}
		if entry == "subdir/" {
			foundSubdir = true
		}
	}
	if !foundTest || !foundSubdir {
		t.Fatalf("ListDir entries incorrect: %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, nil, "../../../../etc/passwd")
	if err == nil {
		t.Fatalf("Expected error for path traversal, got none")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewFileSystemProvider(false, tempDir)
	ctx := context.Background()

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}

	// Test missing claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("should fail"))
	if err == nil {
		t.Fatalf("Expected error for missing claims, got none")
	}

	// Test WriteFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("tenant data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written in tenant subfolder
	expectedPath := filepath.Join(tempDir, "tenant-123", "test.txt")
	_, err = os.Stat(expectedPath)
	if err != nil {
		t.Fatalf("File not found in expected tenant directory: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "tenant data" {
		t.Fatalf("Expected 'tenant data', got '%s'", string(data))
	}

	// Test Path Traversal escaping tenant boundary
	_, err = provider.ReadFile(ctx, claims, "../other-tenant/test.txt")
	if err == nil {
		t.Fatalf("Expected error for cross-tenant path traversal, got none")
	}
}
