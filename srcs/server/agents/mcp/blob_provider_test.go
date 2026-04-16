package mcp

import (
	"context"
	"os"
	"testing"
)

func TestBlobProvider_Local(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
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
	defer os.Unsetenv("OHC_MULTITENANT")

	provider, err := NewBlobProvider()
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()
	key := "test_blob_s3.txt"
	data := []byte("hello s3")

	err = provider.WriteBlob(ctx, key, data)
	if err != nil {
		t.Fatalf("Failed to write blob: %v", err)
	}

	readData, err := provider.ReadBlob(ctx, key)
	if err != nil {
		t.Fatalf("Failed to read blob: %v", err)
	}

	if string(readData) != "stub data" {
		t.Errorf("Expected 'stub data', got %s", string(readData))
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
