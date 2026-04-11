package mcp

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

	provider := NewLocalFSProvider(tempDir)
	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test WriteFile and ReadFile
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(content))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, nil, "../out_of_bounds.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing out of bounds")
	}

	// Test ListDir
	err = provider.WriteFile(ctx, nil, "dir1/file1.txt", []byte("file1"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, nil, "dir1")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "file1.txt" {
		t.Errorf("expected ['file1.txt'], got %v", entries)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false")
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test without claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("fail"))
	if err == nil {
		t.Errorf("expected error writing without claims")
	}

	// Test WriteFile and ReadFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation in filesystem
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Errorf("file was not written to tenant directory")
	}

	content, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(content))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, claims, "../out_of_bounds.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing out of bounds")
	}

	// Test cross-tenant access
	otherClaims := &auth.Claims{OrganizationID: "tenant-2"}
	_, err = provider.ReadFile(ctx, otherClaims, "test.txt")
	if err == nil {
		t.Errorf("expected error reading other tenant's file")
	}
}
