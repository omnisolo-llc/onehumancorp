package mcp

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

	// Test write and read within bounds
	testFile := "test.txt"
	testContent := []byte("hello local")

	if err := provider.WriteFile(ctx, testFile, testContent); err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	data, err := provider.ReadFile(ctx, testFile)
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %s, got %s", testContent, data)
	}

	// Test list directory
	names, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(names) != 1 || names[0] != testFile {
		t.Errorf("expected to find %s, got %v", testFile, names)
	}

	// Test writing out of bounds
	if err := provider.WriteFile(ctx, "../escape.txt", testContent); err == nil {
		t.Errorf("expected error writing outside workspace bounds")
	}

	// Test reading out of bounds
	if _, err := provider.ReadFile(ctx, "../escape.txt"); err == nil {
		t.Errorf("expected error reading outside workspace bounds")
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

	orgID := "tenant-abc"
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: orgID})

	// Test write and read
	testFile := "tenant_data.txt"
	testContent := []byte("hello cloud")

	if err := provider.WriteFile(ctx, testFile, testContent); err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	data, err := provider.ReadFile(ctx, testFile)
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %s, got %s", testContent, data)
	}

	// Verify file is in correct tenant directory on disk
	expectedDiskPath := filepath.Join(tempDir, orgID, testFile)
	if _, err := os.Stat(expectedDiskPath); os.IsNotExist(err) {
		t.Errorf("expected file to exist at %s, but it didn't", expectedDiskPath)
	}

	// Test list directory
	names, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(names) != 1 || names[0] != testFile {
		t.Errorf("expected to find %s, got %v", testFile, names)
	}

	// Test missing tenant context
	emptyCtx := context.Background()
	if err := provider.WriteFile(emptyCtx, testFile, testContent); err == nil {
		t.Errorf("expected error when claims are missing from context")
	}

	// Test writing out of bounds
	if err := provider.WriteFile(ctx, "../escape.txt", testContent); err == nil {
		t.Errorf("expected error writing outside tenant bounds")
	}

	// Test reading out of bounds
	if _, err := provider.ReadFile(ctx, "../escape.txt"); err == nil {
		t.Errorf("expected error reading outside tenant bounds")
	}
}
