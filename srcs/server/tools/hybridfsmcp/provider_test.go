package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	baseDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(baseDir)

	provider := NewLocalFSProvider(baseDir)
	ctx := context.Background()

	t.Run("Write and Read", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
		if err != nil {
			t.Fatal(err)
		}
		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatal(err)
		}
		if string(data) != "hello" {
			t.Errorf("expected 'hello', got '%s'", string(data))
		}
	})

	t.Run("Directory Traversal", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		if err == nil {
			t.Error("expected error for directory traversal")
		}
	})

    t.Run("ListDir", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test1.txt", []byte("hello"))
		if err != nil {
			t.Fatal(err)
		}
        err = provider.WriteFile(ctx, "test2.txt", []byte("world"))
		if err != nil {
			t.Fatal(err)
		}
        files, err := provider.ListDir(ctx, ".")
        if err != nil {
            t.Fatal(err)
        }
        if len(files) < 2 {
            t.Errorf("Expected at least 2 files, got %d", len(files))
        }
    })
}

func TestCloudFSProvider(t *testing.T) {
	baseDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(baseDir)

	provider := NewCloudFSProvider(baseDir)
	claims := &auth.Claims{OrganizationID: "org123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	t.Run("Write and Read", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
		if err != nil {
			t.Fatal(err)
		}
		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatal(err)
		}
		if string(data) != "hello cloud" {
			t.Errorf("expected 'hello cloud', got '%s'", string(data))
		}

		// verify it's under org folder
		_, err = os.Stat(filepath.Join(baseDir, "org123", "test.txt"))
		if err != nil {
			t.Errorf("file not written in correct tenant folder")
		}
	})

	t.Run("No Context", func(t *testing.T) {
		err := provider.WriteFile(context.Background(), "test.txt", []byte("bad"))
		if err == nil {
			t.Error("expected error when no claims in context")
		}
	})

    t.Run("Directory Traversal", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		if err == nil {
			t.Error("expected error for directory traversal")
		}
	})
}
