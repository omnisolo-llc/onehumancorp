package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}
	ctx := context.Background()

	// Test Write and Read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test bounds check (escape attempts)
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error when path escapes base dir")
	}
    err = provider.WriteFile(ctx, "/tmp/escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for absolute path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &CloudFSProvider{TenantBaseDir: tmpDir}

	// Unauthenticated should fail
	ctx := context.Background()
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error for unauthenticated access")
	}

	// Authenticated
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test Write and Read
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it wrote to the correct tenant dir
	tenantDirPath := filepath.Join(tmpDir, "tenant-1")
	if _, err := os.Stat(filepath.Join(tenantDirPath, "test.txt")); os.IsNotExist(err) {
		t.Errorf("file was not written to tenant directory")
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test bounds check
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error when path escapes base dir")
	}
    err = provider.WriteFile(ctx, "/tmp/escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error for absolute path")
	}
}
