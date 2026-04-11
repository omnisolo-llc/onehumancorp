package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfsprovider")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test boundary checking for WriteFile
	t.Run("WriteFile absolute path", func(t *testing.T) {
		err := provider.WriteFile(ctx, "/tmp/somefile.txt", []byte("data"))
		if err != ErrInvalidPath {
			t.Errorf("expected ErrInvalidPath, got %v", err)
		}
	})

	t.Run("WriteFile escape boundary", func(t *testing.T) {
		err := provider.WriteFile(ctx, "../escape.txt", []byte("data"))
		if err != ErrAccessDenied {
			t.Errorf("expected ErrAccessDenied, got %v", err)
		}
	})

	t.Run("WriteFile success", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
	})

	t.Run("WriteFile overlap boundary check", func(t *testing.T) {
		// E.g. baseDir = "/tmp/dir", attack = "../dir-other/test.txt"
		// If joined, it might be "/tmp/dir-other/test.txt"
		// If strings.HasPrefix check is just `strings.HasPrefix(target, base)`, it would pass.
		// Our implementation uses `strings.HasPrefix(target, base+string(filepath.Separator))`.
		// Let's create an actual overlapping directory scenario to test the logic manually if needed,
		// but since we can't easily control the tempdir name exactly, we just test normal escape again.
		err := provider.WriteFile(ctx, "../"+filepath.Base(tmpDir)+"-evil/test.txt", []byte("data"))
		if err != ErrAccessDenied {
			t.Errorf("expected ErrAccessDenied for overlap test, got %v", err)
		}
	})

	t.Run("ReadFile success", func(t *testing.T) {
		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
		if string(data) != "hello" {
			t.Errorf("expected 'hello', got %s", string(data))
		}
	})

	t.Run("ListDir success", func(t *testing.T) {
		infos, err := provider.ListDir(ctx, ".")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
		if len(infos) != 1 || infos[0].Name != "test.txt" {
			t.Errorf("expected 1 file 'test.txt', got %v", infos)
		}
	})
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfsprovider")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	t.Run("No tenant", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "test.txt")
		if err != ErrNoTenant {
			t.Errorf("expected ErrNoTenant, got %v", err)
		}
	})

	t.Run("With tenant", func(t *testing.T) {
		claims := &auth.Claims{
			Subject: "tenant1",
		}
		ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

		err := provider.WriteFile(ctxWithClaims, "tenant_file.txt", []byte("cloud"))
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}

		data, err := provider.ReadFile(ctxWithClaims, "tenant_file.txt")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
		if string(data) != "cloud" {
			t.Errorf("expected 'cloud', got %s", string(data))
		}

		// Verify it was written to the tenant subfolder
		tenantPath := filepath.Join(tmpDir, "tenant1", "tenant_file.txt")
		_, err = os.Stat(tenantPath)
		if err != nil {
			t.Errorf("expected file at %s, got err: %v", tenantPath, err)
		}
	})
}
