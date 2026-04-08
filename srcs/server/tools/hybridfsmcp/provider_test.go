package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test write and read
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got %s", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, "dir/file1.txt", []byte("1"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	err = provider.WriteFile(ctx, "dir/file2.txt", []byte("2"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, "dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	// Test directory traversal bounds
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Error("expected error reading outside workspace, got nil")
	}

	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error writing outside workspace, got nil")
	}

	// Check IsLocal
	if !provider.IsLocal() {
		t.Error("expected IsLocal to be true")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Context without claims
	ctx := context.Background()

	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Error("expected error without claims, got nil")
	}

	// Context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test write and read
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(data))
	}

	// Verify file is correctly placed in tenant sub-directory
	tenantDir := filepath.Join(tmpDir, "tenant-123")
	b, err := os.ReadFile(filepath.Join(tenantDir, "test.txt"))
	if err != nil {
		t.Fatalf("failed to read actual file: %v", err)
	}
	if string(b) != "hello cloud" {
		t.Errorf("unexpected file content: %s", string(b))
	}

	// Test directory traversal bounds
	_, err = provider.ReadFile(ctxWithClaims, "../outside.txt")
	if err == nil {
		t.Error("expected error reading outside workspace, got nil")
	}

	// Check IsLocal
	if provider.IsLocal() {
		t.Error("expected IsLocal to be false")
	}
}
