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
    provider := NewFileSystemProvider(true, tmpDir)
    ctx := context.Background()

    err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil || string(data) != "hello" {
        t.Fatalf("ReadFile failed or mismatch: %v", err)
    }

    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
    if err == nil {
        t.Fatal("Expected error for directory traversal")
    }

    entries, err := provider.ListDir(ctx, "")
    if err != nil || len(entries) != 1 || entries[0] != "test.txt" {
        t.Fatalf("ListDir failed or mismatch: %v, entries: %v", err, entries)
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewFileSystemProvider(false, tmpDir)
    claims := &auth.Claims{OrganizationID: "tenant1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    err := provider.WriteFile(ctx, "cloud.txt", []byte("cloud"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    data, err := provider.ReadFile(ctx, "cloud.txt")
    if err != nil || string(data) != "cloud" {
        t.Fatalf("ReadFile failed or mismatch: %v", err)
    }

    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
    if err == nil {
        t.Fatal("Expected error for directory traversal")
    }

    _, err = os.Stat(filepath.Join(tmpDir, "tenant1", "cloud.txt"))
    if err != nil {
        t.Fatalf("File not created in tenant dir: %v", err)
    }

    ctxNoAuth := context.Background()
    err = provider.WriteFile(ctxNoAuth, "cloud.txt", []byte("fail"))
    if err == nil {
        t.Fatal("Expected error without auth claims")
    }
}

func TestFSMCP(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewFileSystemProvider(true, tmpDir)
    mcp := NewFSMCP(provider)
    ctx := context.Background()

    tools := mcp.ListTools()
    if len(tools) != 3 {
        t.Fatalf("Expected 3 tools")
    }

    _, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "mcp.txt", "content": "mcp"})
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "mcp.txt"})
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }
    m := res.(map[string]interface{})
    if m["content"] != "mcp" {
        t.Fatalf("Content mismatch")
    }

    resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": ""})
    if err != nil {
        t.Fatalf("CallTool list_directory failed: %v", err)
    }
    mList := resList.(map[string]interface{})
    entries := mList["entries"].([]string)
    if len(entries) != 1 || entries[0] != "mcp.txt" {
        t.Fatalf("Entries mismatch")
    }
}
