package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("ReadFile got %q, want %q", string(data), "hello world")
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Errorf("Expected directory traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello tenant"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello tenant" {
		t.Errorf("ReadFile got %q, want %q", string(data), "hello tenant")
	}

	// Test missing claims
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Errorf("Expected unauthorized error, got nil")
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Errorf("Expected directory traversal error, got nil")
	}
}
