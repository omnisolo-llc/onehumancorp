package hybridfsmcp

import (
    "context"
    "os"
    "testing"
)

func TestLocalFSProvider(t *testing.T) {
    baseDir, err := os.MkdirTemp("", "localfstest")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(baseDir)

    provider := NewLocalFSProvider(baseDir)

    ctx := context.Background()

    err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := provider.ReadFile(ctx, nil, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Errorf("expected hello, got %s", string(data))
    }

    files, err := provider.ListDir(ctx, nil, "")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(files) != 1 || files[0] != "test.txt" {
        t.Errorf("unexpected files: %v", files)
    }

    // Test path escape
    err = provider.WriteFile(ctx, nil, "../escape.txt", []byte("bad"))
    if err == nil {
        t.Error("expected error writing to escaped path")
    }
}
