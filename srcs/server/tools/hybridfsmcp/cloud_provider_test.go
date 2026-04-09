package hybridfsmcp

import (
	"context"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudFSProvider_ResolvePath(t *testing.T) {
	provider := NewCloudFSProvider("/base")
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	// Test case: absolute path
	_, err := provider.resolvePath(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("expected error for absolute path")
	}

	// Test case: path without organization prefix
	path, err := provider.resolvePath(ctx, "test.txt")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	expected := filepath.Join("/base", "org-123", "test.txt")
	if path != expected {
		t.Errorf("expected %s, got %s", expected, path)
	}

	// Test case: path with organization prefix
	path, err = provider.resolvePath(ctx, "org-123/test.txt")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	expected = filepath.Join("/base", "org-123", "test.txt")
	if path != expected {
		t.Errorf("expected %s, got %s", expected, path)
	}

	// Test case: path is just the organization prefix
	path, err = provider.resolvePath(ctx, "org-123")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	expected = filepath.Join("/base", "org-123")
	if path != expected {
		t.Errorf("expected %s, got %s", expected, path)
	}
}

func TestCloudFSProvider_MissingClaims(t *testing.T) {
	provider := NewCloudFSProvider("/base")
	_, err := provider.resolvePath(context.Background(), "test.txt")
	if err == nil {
		t.Errorf("expected error for missing claims")
	}
}

func TestCloudFSProvider_WriteReadList(t *testing.T) {
	dir := t.TempDir()
	provider := NewCloudFSProvider(dir)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	// ListDir expects the scoped path
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}
}
