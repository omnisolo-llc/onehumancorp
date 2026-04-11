package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	if !provider.IsLocal() {
		t.Errorf("Expected IsLocal to be true")
	}

	t.Run("Write and Read File", func(t *testing.T) {
		content := []byte("hello local")
		err := provider.WriteFile(ctx, "test.txt", content)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		readContent, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("Failed to read file: %v", err)
		}

		if string(readContent) != "hello local" {
			t.Errorf("Expected 'hello local', got '%s'", string(readContent))
		}
	})

	t.Run("List Directory", func(t *testing.T) {
		provider.WriteFile(ctx, "dir/file1.txt", []byte("1"))
		provider.WriteFile(ctx, "dir/file2.txt", []byte("2"))

		entries, err := provider.ListDir(ctx, "dir")
		if err != nil {
			t.Fatalf("Failed to list directory: %v", err)
		}

		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}
	})

	t.Run("Path Traversal Prevention", func(t *testing.T) {
		err := provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
		if err == nil {
			t.Errorf("Expected error for path traversal, got nil")
		}

		_, err = provider.ReadFile(ctx, "/etc/passwd")
		if err == nil {
			t.Errorf("Expected error for absolute path, got nil")
		}
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	// Create context with tenant ID
	tenantID := "tenant-123"
	claims := &auth.Claims{OrganizationID: tenantID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	if provider.IsLocal() {
		t.Errorf("Expected IsLocal to be false")
	}

	t.Run("Missing Tenant Claims", func(t *testing.T) {
		emptyCtx := context.Background()
		err := provider.WriteFile(emptyCtx, "test.txt", []byte("bad"))
		if err == nil {
			t.Errorf("Expected error for missing tenant claims, got nil")
		}
	})

	t.Run("Write and Read File Scoped", func(t *testing.T) {
		content := []byte("hello cloud")
		err := provider.WriteFile(ctx, "test.txt", content)
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		readContent, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("Failed to read file: %v", err)
		}

		if string(readContent) != "hello cloud" {
			t.Errorf("Expected 'hello cloud', got '%s'", string(readContent))
		}

		// Verify it was written to the tenant subfolder
		tenantFile := filepath.Join(tempDir, tenantID, "test.txt")
		if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
			t.Errorf("Expected file to exist at %s", tenantFile)
		}
	})

	t.Run("List Directory", func(t *testing.T) {
		provider.WriteFile(ctx, "dir/file1.txt", []byte("1"))
		provider.WriteFile(ctx, "dir/file2.txt", []byte("2"))

		entries, err := provider.ListDir(ctx, "dir")
		if err != nil {
			t.Fatalf("Failed to list directory: %v", err)
		}

		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}
	})

	t.Run("Path Traversal Prevention", func(t *testing.T) {
		err := provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
		if err == nil {
			t.Errorf("Expected error for path traversal, got nil")
		}

		_, err = provider.ReadFile(ctx, "/etc/passwd")
		if err == nil {
			t.Errorf("Expected error for absolute path, got nil")
		}
	})
}
