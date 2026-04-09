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
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewFileSystemProvider(true, tmpDir).(*LocalFSProvider)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("local data"))
	if err != nil {
		t.Fatal(err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "local data" {
		t.Errorf("expected 'local data', got %s", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test Path Traversal Protection
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Fatal("expected error for path traversal")
	}

	err = provider.WriteFile(ctx, nil, "../../outside.txt", []byte("bad"))
	if err == nil {
		t.Fatal("expected error for path traversal")
	}

	_, err = provider.ListDir(ctx, nil, "..")
	if err == nil {
		t.Fatal("expected error for path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfsprovider")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewFileSystemProvider(false, tmpDir).(*CloudFSProvider)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("data"))
	if err == nil {
		t.Fatal("expected error for missing claims")
	}

	// Test WriteFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("cloud data"))
	if err != nil {
		t.Fatal(err)
	}

	// Verify it was written to tenant directory
	tenantFile := filepath.Join(tmpDir, "tenant-1", "test.txt")
	if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
		t.Fatalf("expected file to exist at %s", tenantFile)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "cloud data" {
		t.Errorf("expected 'cloud data', got %s", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test Path Traversal Protection
	_, err = provider.ReadFile(ctx, claims, "../outside.txt")
	if err == nil {
		t.Fatal("expected error for path traversal")
	}

	err = provider.WriteFile(ctx, claims, "../../outside.txt", []byte("bad"))
	if err == nil {
		t.Fatal("expected error for path traversal")
	}

	_, err = provider.ListDir(ctx, claims, "..")
	if err == nil {
		t.Fatal("expected error for path traversal")
	}
}
