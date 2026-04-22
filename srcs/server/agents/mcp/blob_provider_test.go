package mcp

import (
	"context"
	"os"
	"testing"
)

func TestBlobProvider_Local(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_STANDALONE")

    dir, err := os.MkdirTemp("", "blob_provider_test")
    if err != nil {
        t.Fatalf("Failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(dir)

    // Override the default path by directly testing LocalBlobProvider
	provider, err := NewLocalBlobProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()
	key := "test_blob.txt"
	data := []byte("hello local")

	err = provider.WriteBlob(ctx, key, data)
	if err != nil {
		t.Fatalf("Failed to write blob: %v", err)
	}

	readData, err := provider.ReadBlob(ctx, key)
	if err != nil {
		t.Fatalf("Failed to read blob: %v", err)
	}

	if string(readData) != "hello local" {
		t.Errorf("Expected 'hello local', got %s", string(readData))
	}
}

func TestBlobProvider_S3(t *testing.T) {
	os.Unsetenv("OHC_STANDALONE")
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("S3_ENDPOINT", "localhost:9000")
	os.Setenv("S3_ACCESS_KEY", "minioadmin")
	os.Setenv("S3_SECRET_KEY", "minioadmin")
	defer func() {
		os.Unsetenv("OHC_MULTITENANT")
		os.Unsetenv("S3_ENDPOINT")
		os.Unsetenv("S3_ACCESS_KEY")
		os.Unsetenv("S3_SECRET_KEY")
	}()

	provider, err := NewBlobProvider()
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	// Because we can't easily start a real S3 server in this test,
	// we just test that the correct provider type was created.
	_, ok := provider.(*S3BlobProvider)
	if !ok {
		t.Fatalf("Expected *S3BlobProvider, got %T", provider)
	}
}
func TestLocalBlobProvider_PathTraversal(t *testing.T) {
    dir, err := os.MkdirTemp("", "blob_provider_test")
    if err != nil {
        t.Fatalf("Failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(dir)

    provider, err := NewLocalBlobProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
    ctx := context.Background()

    err = provider.WriteBlob(ctx, "../../../etc/passwd", []byte("hacked"))
    if err == nil {
        t.Errorf("Expected error for path traversal, got nil")
    }

    err = provider.WriteBlob(ctx, "/etc/passwd", []byte("hacked"))
    if err == nil {
        t.Errorf("Expected error for absolute path, got nil")
    }
}
