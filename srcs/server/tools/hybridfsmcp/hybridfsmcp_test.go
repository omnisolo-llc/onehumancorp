package hybridfsmcp

import (
    "context"
    "os"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
    tmpDir, err := os.MkdirTemp("", "localfs")
    if err != nil {
        t.Fatal(err)
    }
    defer os.RemoveAll(tmpDir)

    provider := NewFileSystemProvider(true, tmpDir)

    ctx := context.Background()

    // Test write
    err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatal(err)
    }

    // Test read
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatal(err)
    }
    if string(data) != "hello" {
        t.Errorf("expected hello, got %s", string(data))
    }

    // Test list
    infos, err := provider.ListDir(ctx, ".")
    if err != nil {
        t.Fatal(err)
    }
    if len(infos) != 1 || infos[0].Name() != "test.txt" {
        t.Errorf("unexpected list result")
    }

    // Test search
    matches, err := provider.SearchFiles(ctx, ".", "test")
    if err != nil {
        t.Fatal(err)
    }
    if len(matches) != 1 || matches[0] != "test.txt" {
        t.Errorf("unexpected search result")
    }

    // Test path bounding
    err = provider.WriteFile(ctx, "../test.txt", []byte("bad"))
    if err == nil {
        t.Error("expected error for path bounding")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir, err := os.MkdirTemp("", "cloudfs")
    if err != nil {
        t.Fatal(err)
    }
    defer os.RemoveAll(tmpDir)

    provider := NewFileSystemProvider(false, tmpDir)

    claims := &auth.Claims{OrganizationID: "tenant1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Test write
    err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
    if err != nil {
        t.Fatal(err)
    }

    // Test read
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatal(err)
    }
    if string(data) != "hello cloud" {
        t.Errorf("expected hello cloud, got %s", string(data))
    }

    // Test path bounding
    err = provider.WriteFile(ctx, "../test.txt", []byte("bad"))
    if err == nil {
        t.Error("expected error for path bounding")
    }

    // Test no claims
    ctxNoClaims := context.Background()
    err = provider.WriteFile(ctxNoClaims, "test.txt", []byte("bad"))
    if err == nil {
        t.Error("expected error for missing claims")
    }
}

func TestServer(t *testing.T) {
    tmpDir, err := os.MkdirTemp("", "serverfs")
    if err != nil {
        t.Fatal(err)
    }
    defer os.RemoveAll(tmpDir)

    provider := NewFileSystemProvider(true, tmpDir)
    server := NewServer(provider)

    ctx := context.Background()

    tools := server.ListTools()
    if len(tools) != 4 {
        t.Errorf("expected 4 tools")
    }

    writeArgs := map[string]interface{}{"path": "test.txt", "content": "server hello"}
    _, err = server.CallTool(ctx, "write_file", writeArgs)
    if err != nil {
        t.Fatal(err)
    }

    readArgs := map[string]interface{}{"path": "test.txt"}
    res, err := server.CallTool(ctx, "read_file", readArgs)
    if err != nil {
        t.Fatal(err)
    }
    if res.(string) != "server hello" {
        t.Errorf("expected server hello, got %v", res)
    }
}
