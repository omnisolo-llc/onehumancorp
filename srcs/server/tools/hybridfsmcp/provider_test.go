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

    // Test writing a file
    err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
    if err != nil {
        t.Fatalf("failed to write file: %v", err)
    }

    // Test reading the file
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("failed to read file: %v", err)
    }
    if string(data) != "hello world" {
        t.Errorf("expected 'hello world', got '%s'", string(data))
    }

    // Test path traversal protection
    err = provider.WriteFile(ctx, "../outside.txt", []byte("malicious"))
    if err == nil {
        t.Errorf("expected error when writing outside base dir")
    }

    // Test listing directory
    infos, err := provider.ListDir(ctx, ".")
    if err != nil {
        t.Fatalf("failed to list dir: %v", err)
    }
    if len(infos) != 1 || infos[0].Name != "test.txt" {
        t.Errorf("unexpected directory contents: %+v", infos)
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewCloudFSProvider(tmpDir)

    claims := &auth.Claims{
        OrganizationID: "tenant-123",
    }
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Test writing a file
    err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
    if err != nil {
        t.Fatalf("failed to write file: %v", err)
    }

    // Verify it was written to the correct tenant directory
    tenantPath := filepath.Join(tmpDir, "tenants", "tenant-123", "test.txt")
    if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
        t.Errorf("file was not written to tenant directory")
    }

    // Test reading the file
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("failed to read file: %v", err)
    }
    if string(data) != "hello cloud" {
        t.Errorf("expected 'hello cloud', got '%s'", string(data))
    }

    // Test path traversal protection
    err = provider.WriteFile(ctx, "../outside.txt", []byte("malicious"))
    if err == nil {
        t.Errorf("expected error when writing outside tenant dir")
    }

    // Test access without claims
    ctxNoClaims := context.Background()
    err = provider.WriteFile(ctxNoClaims, "test.txt", []byte("data"))
    if err == nil {
        t.Errorf("expected error without claims")
    }
}

func TestFactory(t *testing.T) {
    // Test Cloud mode
    t.Setenv("OHC_MULTITENANT", "true")
    t.Setenv("OHC_FS_ROOT", "/tmp/cloud")
    p1 := NewProvider()
    if _, ok := p1.(*CloudFSProvider); !ok {
        t.Errorf("expected CloudFSProvider in multitenant mode")
    }

    // Test Local mode
    t.Setenv("OHC_MULTITENANT", "false")
    t.Setenv("OHC_FS_ROOT", "/tmp/local")
    p2 := NewProvider()
    if _, ok := p2.(*LocalFSProvider); !ok {
        t.Errorf("expected LocalFSProvider in standalone mode")
    }
}
