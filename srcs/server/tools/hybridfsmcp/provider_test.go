package hybridfsmcp

import (
    "context"
    "path/filepath"
    "testing"
)

func TestLocalFSProvider(t *testing.T) {
    dir := t.TempDir()
    p, err := NewLocalFSProvider(dir)
    if err != nil {
        t.Fatalf("failed to create provider: %v", err)
    }
    if !p.IsLocal() {
        t.Fatalf("expected true")
    }
    ctx := context.Background()

    err = p.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := p.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Fatalf("unexpected content")
    }

    entries, err := p.ListDir(ctx, "")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(entries) != 1 || entries[0]["name"] != "test.txt" {
        t.Fatalf("unexpected dir entries: %v", entries)
    }
}

func TestCloudFSProvider(t *testing.T) {
    dir := t.TempDir()
    p, err := NewCloudFSProvider(dir)
    if err != nil {
        t.Fatalf("failed to create provider: %v", err)
    }
    if p.IsLocal() {
        t.Fatalf("expected false")
    }
    ctx := context.Background()

    err = p.WriteFile(ctx, filepath.Join("org1", "test.txt"), []byte("cloud"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := p.ReadFile(ctx, filepath.Join("org1", "test.txt"))
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "cloud" {
        t.Fatalf("unexpected content")
    }

    entries, err := p.ListDir(ctx, "org1")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(entries) != 1 || entries[0]["name"] != "test.txt" {
        t.Fatalf("unexpected dir entries: %v", entries)
    }
}
