package mcp

import (
    "context"
    "os"
    "path/filepath"
    "testing"
)

func TestLocalFSProvider(t *testing.T) {
    tempDir, err := os.MkdirTemp("", "localfs")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(tempDir)

    // Create absolute temp dir to match filepath.Abs resolution logic
    absTempDir, _ := filepath.Abs(tempDir)
    provider := NewLocalFSProvider(absTempDir)
    ctx := context.Background()

    // Test WriteFile
    err = provider.WriteFile(ctx, "test.txt", []byte("hello"), nil)
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Test ReadFile
    data, err := provider.ReadFile(ctx, "test.txt", nil)
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Errorf("expected 'hello', got %q", string(data))
    }

    // Test ListDir
    names, err := provider.ListDir(ctx, ".", nil)
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(names) != 1 || names[0] != "test.txt" {
        t.Errorf("expected ['test.txt'], got %v", names)
    }

    // Test escape bounds
    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"), nil)
    if err == nil {
        t.Error("expected error for escaping workspace bounds")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tempDir, err := os.MkdirTemp("", "cloudfs")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(tempDir)

    absTempDir, _ := filepath.Abs(tempDir)
    provider := NewCloudFSProvider(absTempDir)
    ctx := context.Background()
    claims := map[string]interface{}{"tenant_id": "tenant1"}

    // Test WriteFile
    err = provider.WriteFile(ctx, "test.txt", []byte("hello"), claims)
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Test ReadFile
    data, err := provider.ReadFile(ctx, "test.txt", claims)
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Errorf("expected 'hello', got %q", string(data))
    }

    // Test missing tenant ID
    err = provider.WriteFile(ctx, "test2.txt", []byte("bad"), nil)
    if err == nil {
        t.Error("expected error for missing tenant_id")
    }

    // Test escape bounds
    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"), claims)
    if err == nil {
        t.Error("expected error for escaping tenant bounds")
    }
}

func TestNewFSProvider(t *testing.T) {
    p := NewFSProvider("OHC_MULTITENANT", "/tmp")
    if _, ok := p.(*CloudFSProvider); !ok {
        t.Error("expected CloudFSProvider")
    }

    p2 := NewFSProvider("OHC_STANDALONE", "/tmp")
    if _, ok := p2.(*LocalFSProvider); !ok {
        t.Error("expected LocalFSProvider")
    }
}
