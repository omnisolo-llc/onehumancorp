package hybridfsmcp

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewLocalFSProvider(tmpDir)

    ctx := context.Background()

    // Write
    err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Read
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if string(data) != "hello" {
        t.Errorf("expected hello, got %s", string(data))
    }

    // List
    entries, err := provider.ListDir(ctx, ".")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(entries) != 1 || entries[0] != "test.txt" {
        t.Errorf("expected [test.txt], got %v", entries)
    }

    // Absolute path
    err = provider.WriteFile(ctx, "/etc/passwd", []byte("hack"))
    if err == nil {
        t.Errorf("expected error for absolute path")
    }

    // Traversal
    err = provider.WriteFile(ctx, "../hack.txt", []byte("hack"))
    if err == nil {
        t.Errorf("expected error for traversal")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewCloudFSProvider(tmpDir)

    claims := &auth.Claims{OrganizationID: "org-123"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Write
    err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Read
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if string(data) != "hello cloud" {
        t.Errorf("expected hello cloud, got %s", string(data))
    }

    // Tenant separation
    claims2 := &auth.Claims{OrganizationID: "org-456"}
    ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)

    _, err = provider.ReadFile(ctx2, "test.txt")
    if err == nil {
        t.Errorf("expected error reading other tenant file")
    }
}
