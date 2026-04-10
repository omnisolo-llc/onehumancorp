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

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("unexpected list dir result: %v", entries)
	}

	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatal("expected error on path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test.txt", []byte("cloud"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "cloud" {
		t.Fatalf("expected 'cloud', got '%s'", string(data))
	}

	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Fatal("expected error when no claims present")
	}

    _, err = os.Stat(filepath.Join(tmpDir, "tenant-1", "test.txt"))
    if err != nil {
        t.Fatalf("expected file to exist in tenant dir, error: %v", err)
    }
}

func TestFactory(t *testing.T) {
	tmpDir := t.TempDir()

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	p1 := NewHybridFSProvider(tmpDir)
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatal("expected LocalFSProvider")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p2 := NewHybridFSProvider(tmpDir)
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatal("expected CloudFSProvider")
	}
}
