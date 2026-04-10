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
    provider := &LocalFSProvider{WorkspaceDir: tmpDir}

    ctx := context.Background()

    // Write
    err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Read
    content, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(content) != "hello" {
        t.Errorf("Expected hello, got %s", string(content))
    }

    // List
    entries, err := provider.ListDir(ctx, ".")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(entries) != 1 || entries[0] != "test.txt" {
        t.Errorf("Expected [test.txt], got %v", entries)
    }

    // Out of bounds
    _, err = provider.ReadFile(ctx, "../test.txt")
    if err == nil {
        t.Errorf("Expected bounds error, got nil")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := &CloudFSProvider{BaseDir: tmpDir}

    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Write
    err := provider.WriteFile(ctx, "test.txt", []byte("world"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Read
    content, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(content) != "world" {
        t.Errorf("Expected world, got %s", string(content))
    }

    // Verify path
    if _, err := os.Stat(filepath.Join(tmpDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
        t.Errorf("File not saved in tenant directory")
    }

    // Missing claims
    ctxNoClaims := context.Background()
    _, err = provider.ReadFile(ctxNoClaims, "test.txt")
    if err == nil {
        t.Errorf("Expected missing claims error")
    }
}

func TestServer(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "true")
    defer os.Unsetenv("OHC_STANDALONE")
    server := NewServer()

    if len(server.ListTools()) != 3 {
        t.Errorf("Expected 3 tools")
    }

    if server.Provider == nil {
        t.Errorf("Provider is nil")
    }

    // Call Tools with missing args
    ctx := context.Background()
    _, err := server.CallTool(ctx, "read_file", map[string]interface{}{})
    if err == nil {
        t.Errorf("Expected error for missing args")
    }

    _, err = server.CallTool(ctx, "unknown_tool", map[string]interface{}{})
    if err == nil {
        t.Errorf("Expected error for unknown tool")
    }
}

func TestServer_Multitenant(t *testing.T) {
    os.Setenv("OHC_MULTITENANT", "true")
    defer os.Unsetenv("OHC_MULTITENANT")
    server := NewServer()
    if _, ok := server.Provider.(*CloudFSProvider); !ok {
        t.Errorf("Expected CloudFSProvider")
    }
}

func TestCloudFSProvider_Bounds(t *testing.T) {
    tmpDir := t.TempDir()
    provider := &CloudFSProvider{BaseDir: tmpDir}

    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    _, err := provider.ReadFile(ctx, "../../../etc/passwd")
    if err == nil {
        t.Errorf("Expected bounds error for path traversal, got nil")
    }
}
