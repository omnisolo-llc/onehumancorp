package hybridfsmcp

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
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	os.Setenv("OHC_WORKSPACE_DIR", dir)
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	provider := NewLocalFSProvider()

	// Test write
	ctx := context.Background()
	err = provider.WriteFile(ctx, "test.txt", "hello world")
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if content != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", content)
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("ListDir unexpected result: %v", entries)
	}

	// Test directory traversal prevention
	err = provider.WriteFile(ctx, "../escape.txt", "escape")
	if err == nil {
		t.Fatalf("Expected error for directory traversal, got nil")
	}

	// Test Absolute Path traversal prevention
	// It's technically safe if we clean it, but our logic forbids starting with / unless it matches
	// the baseDir exactly. Actually we joined it: filepath.Join(baseDir, reqPath).
	// If reqPath is /etc/passwd, Join ignores baseDir and returns /etc/passwd.
	// Our clean logic catches it!
	err = provider.WriteFile(ctx, "/etc/passwd", "escape")
	if err == nil {
		t.Fatalf("Expected error for absolute path, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	os.Setenv("OHC_TENANT_PV_DIR", dir)
	defer os.Unsetenv("OHC_TENANT_PV_DIR")

	provider := NewCloudFSProvider()

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", "cloud hello")
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if content != "cloud hello" {
		t.Fatalf("Expected 'cloud hello', got '%s'", content)
	}

	// Verify file is actually in the tenant subfolder
	b, err := os.ReadFile(filepath.Join(dir, "tenant-1", "test.txt"))
	if err != nil || string(b) != "cloud hello" {
		t.Fatalf("File not in correct tenant dir")
	}

	// Test missing claims
	badCtx := context.Background()
	err = provider.WriteFile(badCtx, "test.txt", "cloud hello")
	if err == nil {
		t.Fatalf("Expected error for missing claims")
	}

	// Directory traversal
	err = provider.WriteFile(ctx, "../escape.txt", "escape")
	if err == nil {
		t.Fatalf("Expected error for directory traversal, got nil")
	}
}
