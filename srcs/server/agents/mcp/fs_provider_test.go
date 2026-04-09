package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathBounding(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)

	ctx := context.Background()

	// Valid path
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error writing valid file: %v", err)
	}

	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(content) != "hello" {
		t.Fatalf("unexpected error reading valid file: %v", err)
	}

	// Invalid absolute path
	if err := provider.WriteFile(ctx, "/etc/passwd", []byte("bad")); err == nil {
		t.Fatal("expected error for absolute path")
	}

	// Invalid directory traversal
	if err := provider.WriteFile(ctx, "../outside.txt", []byte("bad")); err == nil {
		t.Fatal("expected error for directory traversal")
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{OrganizationID: "tenant-a"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Valid path
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error writing valid file: %v", err)
	}

	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(content) != "hello" {
		t.Fatalf("unexpected error reading valid file: %v", err)
	}

	// Check if file is actually stored under tenant-a directory
	if _, err := os.Stat(filepath.Join(tmpDir, "tenant-a", "test.txt")); os.IsNotExist(err) {
		t.Fatal("file was not written to tenant directory")
	}

	// Ensure unauthorized access fails
	ctxUnauth := context.Background()
	if err := provider.WriteFile(ctxUnauth, "test.txt", []byte("bad")); err == nil {
		t.Fatal("expected error for unauthorized access")
	}
}

func TestNewHybridFSServer(t *testing.T) {
	tmpDir := t.TempDir()
	srvLocal := NewHybridFSServer(true, tmpDir)
	if _, ok := srvLocal.Provider().(*LocalFSProvider); !ok {
		t.Fatal("expected LocalFSProvider in standalone mode")
	}

	srvCloud := NewHybridFSServer(false, tmpDir)
	if _, ok := srvCloud.Provider().(*CloudFSProvider); !ok {
		t.Fatal("expected CloudFSProvider in cloud mode")
	}
}
