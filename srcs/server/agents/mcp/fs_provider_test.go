package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider := NewLocalFSProvider(dir)
	ctx := context.Background()

	// 1. Write file
	err = provider.WriteFile(ctx, "hello.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// 2. Read file
	data, err := provider.ReadFile(ctx, "hello.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "world" {
		t.Fatalf("Expected 'world', got %s", string(data))
	}

	// 3. Path traversal attempt
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error for path traversal, got nil")
	}

	// 4. List dir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name != "hello.txt" {
		t.Fatalf("Unexpected ListDir result: %v", infos)
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider := NewCloudFSProvider(dir)

	// Context without claims
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "hello.txt")
	if err == nil {
		t.Fatalf("Expected error for missing claims, got nil")
	}

	// Context with claims
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-1",
	})

	// 1. Write file
	err = provider.WriteFile(ctx, "hello.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation visually on disk
	_, err = os.Stat(filepath.Join(dir, "org-1", "hello.txt"))
	if err != nil {
		t.Fatalf("File not written to correct tenant directory: %v", err)
	}

	// 2. Read file
	data, err := provider.ReadFile(ctx, "hello.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "world" {
		t.Fatalf("Expected 'world', got %s", string(data))
	}

	// 3. Path traversal attempt (trying to escape tenant dir)
	err = provider.WriteFile(ctx, "../org-2/escape.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error for path traversal out of tenant dir, got nil")
	}

	// 4. List dir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name != "hello.txt" {
		t.Fatalf("Unexpected ListDir result: %v", infos)
	}

	// List dir for non-existent path
	infosEmpty, err := provider.ListDir(ctx, "nonexistent")
	if err != nil {
		t.Fatalf("Expected nil error for non-existent dir, got %v", err)
	}
	if len(infosEmpty) != 0 {
		t.Fatalf("Expected empty list, got %v", infosEmpty)
	}
}
