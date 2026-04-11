package mcp

import (
	"context"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ResolvePath(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local fs provider: %v", err)
	}

	tests := []struct {
		name        string
		target      string
		expectError bool
	}{
		{"valid path", "test.txt", false},
		{"valid subpath", "sub/test.txt", false},
		{"valid clean path", "./test.txt", false},
		{"invalid escape", "../test.txt", true},
		{"invalid escape absolute", "/etc/passwd", true}, // clean removes leading / when joined if not careful, but path.Join cleans it to relative to base
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := provider.resolvePath(tt.target)
			if (err != nil) != tt.expectError {
				t.Errorf("resolvePath(%q) error = %v, expectError %v", tt.target, err, tt.expectError)
			}
		})
	}
}

func TestLocalFSProvider_ReadWriteList(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local fs provider: %v", err)
	}

	ctx := context.Background()

	// Write
	err = provider.WriteFile(ctx, "hello.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, "hello.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "world" {
		t.Errorf("expected 'world', got '%s'", string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Errorf("expected 1 entry, got %d", len(entries))
	}
	if entries[0].Name() != "hello.txt" {
		t.Errorf("expected 'hello.txt', got '%s'", entries[0].Name())
	}
}

func TestCloudFSProvider_ResolvePath(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud fs provider: %v", err)
	}

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org123",
	})
	ctxNoClaims := context.Background()

	// 1. Success case
	path, err := provider.resolvePath(ctxWithClaims, "test.txt")
	if err != nil {
		t.Errorf("resolvePath failed for valid target: %v", err)
	}
	if !strings.Contains(path, "org123") {
		t.Errorf("path should contain org id, got: %s", path)
	}

	// 2. Missing claims
	_, err = provider.resolvePath(ctxNoClaims, "test.txt")
	if err == nil {
		t.Errorf("expected error for missing claims")
	}

	// 3. Escape attempt
	_, err = provider.resolvePath(ctxWithClaims, "../escaped.txt")
	if err == nil {
		t.Errorf("expected error for path escape")
	}
}

func TestCloudFSProvider_ReadWriteList(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud fs provider: %v", err)
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org123",
	})

	// Write
	err = provider.WriteFile(ctx, "dir1/hello.txt", []byte("tenant"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, "dir1/hello.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "tenant" {
		t.Errorf("expected 'tenant', got '%s'", string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, "dir1")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Errorf("expected 1 entry, got %d", len(entries))
	}
	if entries[0].Name() != "hello.txt" {
		t.Errorf("expected 'hello.txt', got '%s'", entries[0].Name())
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "factory_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Set env vars
	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	os.Setenv("OHC_MULTITENANT", "true")
	provider, err := NewFileSystemProvider()
	if err != nil {
		t.Fatalf("NewFileSystemProvider failed: %v", err)
	}
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider, got %T", provider)
	}

	os.Setenv("OHC_MULTITENANT", "false")
	provider2, err := NewFileSystemProvider()
	if err != nil {
		t.Fatalf("NewFileSystemProvider failed: %v", err)
	}
	if _, ok := provider2.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider, got %T", provider2)
	}
}
