package mcp

import (
	"context"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)

	ctx := context.Background()

	// Normal write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Normal read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Traversal write attempt
	err = provider.WriteFile(ctx, "../outside.txt", []byte("malicious"))
	if err == nil {
		t.Fatal("expected error on path traversal, got nil")
	}
	if !strings.Contains(err.Error(), "path traversal attempt outside workspace") {
		t.Fatalf("expected traversal error, got: %v", err)
	}

	// Traversal read attempt
	_, err = provider.ReadFile(ctx, "../../etc/passwd")
	if err == nil {
		t.Fatal("expected error on path traversal read, got nil")
	}

	// List dir traversal attempt
	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Fatal("expected error on list dir path traversal, got nil")
	}
}

func TestCloudFSProvider_Isolation(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	ctx := context.Background()

	// Missing claims
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatal("expected error for missing claims, got nil")
	}

	// Valid claims
	claims1 := &auth.Claims{OrganizationID: "tenant1"}
	ctx1 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims1)

	claims2 := &auth.Claims{OrganizationID: "tenant2"}
	ctx2 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims2)

	err = provider.WriteFile(ctx1, "file1.txt", []byte("tenant1 data"))
	if err != nil {
		t.Fatalf("expected no error for tenant1 write, got %v", err)
	}

	// Verify isolation
	_, err = provider.ReadFile(ctx2, "file1.txt")
	if err == nil {
		t.Fatal("expected error for tenant2 reading tenant1 file, got nil")
	}

	// Traversal attempt
	err = provider.WriteFile(ctx1, "../tenant2/file1.txt", []byte("malicious"))
	if err == nil {
		t.Fatal("expected error on path traversal, got nil")
	}
}
