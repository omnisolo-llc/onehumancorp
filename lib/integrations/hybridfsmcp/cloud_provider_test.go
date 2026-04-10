package hybridfsmcp

import (
    "context"
    "os"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudFSProvider(t *testing.T) {
    baseDir, err := os.MkdirTemp("", "cloudfstest")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(baseDir)

    provider := NewCloudFSProvider(baseDir)
    claims := &auth.Claims{OrganizationID: "tenant1"}
    ctx := context.Background()

    err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := provider.ReadFile(ctx, claims, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello cloud" {
        t.Errorf("expected hello cloud, got %s", string(data))
    }

    files, err := provider.ListDir(ctx, claims, "")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(files) != 1 || files[0] != "test.txt" {
        t.Errorf("unexpected files: %v", files)
    }

    // Test without claims
    err = provider.WriteFile(ctx, nil, "test.txt", []byte("bad"))
    if err == nil {
        t.Error("expected error writing without claims")
    }
}
