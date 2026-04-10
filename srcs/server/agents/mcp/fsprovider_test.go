package mcp

import (
	"context"
	"os"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{WorkspaceDir: tmpDir}
	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello local" {
		t.Fatalf("expected 'hello local', got %s", string(data))
	}

	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", files)
	}

    files2, err := provider.ListDir(ctx, "missing_dir")
	if err != nil {
		t.Fatal(err)
	}
	if len(files2) != 0 {
		t.Fatalf("expected empty dir list")
	}

	_, err = provider.ReadFile(ctx, "../out.txt")
	if err == nil {
		t.Fatal("expected error for path traversal")
	}

	_, err = provider.ReadFile(ctx, "/absolute/path")
	if err == nil {
		t.Fatal("expected error for absolute path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &CloudFSProvider{BaseDir: tmpDir}

	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil || err.Error() != "unauthorized: missing tenant organization ID" {
		t.Fatalf("expected unauthorized error, got %v", err)
	}

    // Test that missing auth returns unauthorized for write and list
    err = provider.WriteFile(ctx, "test_cloud.txt", []byte("hello cloud"))
    if err == nil || err.Error() != "unauthorized: missing tenant organization ID" {
		t.Fatalf("expected unauthorized error for write, got %v", err)
	}

    _, err = provider.ListDir(ctx, ".")
    if err == nil || err.Error() != "unauthorized: missing tenant organization ID" {
		t.Fatalf("expected unauthorized error for list, got %v", err)
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	p := NewFileSystemProvider("/tmp")
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Fatal("expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := NewFileSystemProvider("/tmp")
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Fatal("expected LocalFSProvider")
	}
}
