package hybridfsmcp

import (
	"context"
	"os"
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

	provider, err := NewLocalFSProvider()
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Valid path
	err = provider.WriteFile(ctx, nil, "valid.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected no error for valid path, got: %v", err)
	}

	// Invalid paths
	invalidPaths := []string{
		"../outside.txt",
		"nested/../../outside.txt",
		"/absolute/path.txt",
	}

	for _, path := range invalidPaths {
		err = provider.WriteFile(ctx, nil, path, []byte("bad"))
		if err == nil {
			t.Errorf("expected error for path traversal %s, got none", path)
		}
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider, err := NewCloudFSProvider()
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()
	claimsTenant1 := &auth.Claims{OrganizationID: "tenant-1"}
	claimsTenant2 := &auth.Claims{OrganizationID: "tenant-2"}

	// Tenant 1 writes a file
	err = provider.WriteFile(ctx, claimsTenant1, "secret.txt", []byte("tenant 1 data"))
	if err != nil {
		t.Fatalf("expected no error for tenant 1 write, got: %v", err)
	}

	// Tenant 2 tries to read it using traversal
	_, err = provider.ReadFile(ctx, claimsTenant2, "../tenant-1/secret.txt")
	if err == nil {
		t.Errorf("expected error when tenant 2 tries to read tenant 1 data using traversal")
	}

	// Tenant 2 writes its own file
	err = provider.WriteFile(ctx, claimsTenant2, "secret.txt", []byte("tenant 2 data"))
	if err != nil {
		t.Fatalf("expected no error for tenant 2 write, got: %v", err)
	}

	// Verify isolation
	data1, _ := provider.ReadFile(ctx, claimsTenant1, "secret.txt")
	data2, _ := provider.ReadFile(ctx, claimsTenant2, "secret.txt")

	if string(data1) != "tenant 1 data" {
		t.Errorf("tenant 1 data mismatch: %s", string(data1))
	}
	if string(data2) != "tenant 2 data" {
		t.Errorf("tenant 2 data mismatch: %s", string(data2))
	}

	// Test absolute path
	err = provider.WriteFile(ctx, claimsTenant1, "/etc/passwd", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for absolute path")
	}
}

func TestLocalFSProvider_ReadList(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider, err := NewLocalFSProvider()
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	_ = provider.WriteFile(ctx, nil, "file1.txt", []byte("1"))
	_ = provider.WriteFile(ctx, nil, "dir1/file2.txt", []byte("2"))

	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Errorf("failed to list dir: %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	data, err := provider.ReadFile(ctx, nil, "dir1/file2.txt")
	if err != nil {
		t.Errorf("failed to read file: %v", err)
	}
	if string(data) != "2" {
		t.Errorf("expected '2', got %s", string(data))
	}
}
