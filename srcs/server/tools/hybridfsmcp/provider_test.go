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
	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("Expected 'hello', got '%s'", string(data))
	}

	// List dir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("ListDir unexpected output: %v", files)
	}

	// Search files
	matches, err := provider.SearchFiles(ctx, "test")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Fatalf("SearchFiles unexpected output: %v", matches)
	}

	// Path traversal protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseDir := t.TempDir()
	provider := NewCloudFSProvider(baseDir)

	// Create claims for test
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written in tenant dir
	tenantDir := filepath.Join(baseDir, "tenant-1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Fatalf("File was not written in tenant directory")
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Path traversal protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("Expected path traversal error, got nil")
	}

	// Without claims should fail
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Fatalf("Expected auth error, got nil")
	}
}
