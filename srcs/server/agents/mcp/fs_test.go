package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "local_fs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create LocalFSProvider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testData := []byte("hello world")
	if err := provider.WriteFile(ctx, nil, testPath, testData); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, nil, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != string(testData) {
		t.Errorf("Expected %s, got %s", string(testData), string(readData))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, nil, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("Expected 1 entry, got %d", len(entries))
	}
	if entries[0].Name != "test_file.txt" || entries[0].IsDir {
		t.Errorf("Unexpected entry info: %+v", entries[0])
	}

	// Test Path Bounding (Traversal Attempt)
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Error("Expected directory traversal to fail")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create CloudFSProvider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-123"}

	// Ensure no claims fails
	if err := provider.WriteFile(ctx, nil, "test.txt", []byte("data")); err == nil {
		t.Error("Expected write to fail without claims")
	}

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testData := []byte("hello cloud")
	if err := provider.WriteFile(ctx, claims, testPath, testData); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it wrote to the tenant directory
	actualPath := filepath.Join(tmpDir, "tenant-123", "test_dir", "test_file.txt")
	if _, err := os.Stat(actualPath); os.IsNotExist(err) {
		t.Errorf("Expected file to be created at tenant path: %s", actualPath)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, claims, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != string(testData) {
		t.Errorf("Expected %s, got %s", string(testData), string(readData))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, claims, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("Expected 1 entry, got %d", len(entries))
	}
	if entries[0].Name != "test_file.txt" || entries[0].IsDir {
		t.Errorf("Unexpected entry info: %+v", entries[0])
	}

	// Verify isolation
	claims2 := &auth.Claims{OrganizationID: "tenant-456"}
	_, err = provider.ReadFile(ctx, claims2, testPath)
	if err == nil {
		t.Error("Expected tenant-456 to fail reading tenant-123's file")
	}

	// Test Path Bounding
	_, err = provider.ReadFile(ctx, claims, "../test.txt")
	if err == nil {
		t.Error("Expected directory traversal to fail")
	}
}
