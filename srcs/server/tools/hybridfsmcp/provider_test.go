package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write File
	t.Run("WriteFile", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}
	})

	// Read File
	t.Run("ReadFile", func(t *testing.T) {
		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("Failed to read file: %v", err)
		}
		if string(data) != "hello local" {
			t.Errorf("Expected 'hello local', got '%s'", string(data))
		}
	})

	// List Directory
	t.Run("ListDir", func(t *testing.T) {
		err := provider.WriteFile(ctx, "dir/test2.txt", []byte("test"))
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		entries, err := provider.ListDir(ctx, "")
		if err != nil {
			t.Fatalf("Failed to list directory: %v", err)
		}

		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}

		// Simple check, order might vary depending on OS but usually alphabetical
		foundTestTxt := false
		for _, e := range entries {
			if e == "test.txt" {
				foundTestTxt = true
			}
		}
		if !foundTestTxt {
			t.Errorf("Expected 'test.txt' in list, got %v", entries)
		}

		dirEntries, err := provider.ListDir(ctx, "dir")
		if err != nil {
			t.Fatalf("Failed to list subdirectory: %v", err)
		}
		if !reflect.DeepEqual(dirEntries, []string{"test2.txt"}) {
			t.Errorf("Expected ['test2.txt'], got %v", dirEntries)
		}
	})

	// Path Traversal
	t.Run("PathTraversal", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		if err == nil {
			t.Error("Expected error for path traversal on read, got nil")
		}

		err = provider.WriteFile(ctx, "../outside.txt", []byte("test"))
		if err == nil {
			t.Error("Expected error for path traversal on write, got nil")
		}

		_, err = provider.ListDir(ctx, "../")
		if err == nil {
			t.Error("Expected error for path traversal on list, got nil")
		}
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	orgID := "org_123"
	claims := &auth.Claims{OrganizationID: orgID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Unauthorized
	t.Run("Unauthorized", func(t *testing.T) {
		_, err := provider.ReadFile(context.Background(), "test.txt")
		if err == nil {
			t.Error("Expected error for missing claims, got nil")
		}
	})

	// Write File
	t.Run("WriteFile", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		// Verify it was written to the correct tenant directory
		fullPath := filepath.Join(tempDir, orgID, "test.txt")
		_, err = os.Stat(fullPath)
		if os.IsNotExist(err) {
			t.Errorf("Expected file at %s, but it does not exist", fullPath)
		}
	})

	// Read File
	t.Run("ReadFile", func(t *testing.T) {
		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("Failed to read file: %v", err)
		}
		if string(data) != "hello cloud" {
			t.Errorf("Expected 'hello cloud', got '%s'", string(data))
		}
	})

	// List Directory
	t.Run("ListDir", func(t *testing.T) {
		err := provider.WriteFile(ctx, "dir/test2.txt", []byte("test"))
		if err != nil {
			t.Fatalf("Failed to write file: %v", err)
		}

		entries, err := provider.ListDir(ctx, "")
		if err != nil {
			t.Fatalf("Failed to list directory: %v", err)
		}

		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}
	})

	// Path Traversal
	t.Run("PathTraversal", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		if err == nil {
			t.Error("Expected error for path traversal on read, got nil")
		}

		err = provider.WriteFile(ctx, "../outside.txt", []byte("test"))
		if err == nil {
			t.Error("Expected error for path traversal on write, got nil")
		}

		_, err = provider.ListDir(ctx, "../")
		if err == nil {
			t.Error("Expected error for path traversal on list, got nil")
		}
	})
}

func TestNewFileSystemProvider(t *testing.T) {
	local := NewFileSystemProvider(true, "/tmp")
	if _, ok := local.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", local)
	}

	cloud := NewFileSystemProvider(false, "/tmp")
	if _, ok := cloud.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", cloud)
	}
}
