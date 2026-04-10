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

    // Test write
    err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Test read
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Errorf("Expected 'hello', got %s", string(data))
    }

    // Test list
    entries, err := provider.ListDir(ctx, ".")
    if err != nil {
        t.Fatalf("ListDir failed: %v", err)
    }
    if len(entries) != 1 || entries[0].Name != "test.txt" {
        t.Errorf("ListDir failed, expected test.txt")
    }

    // Test traversal
    err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
    if err == nil {
        t.Error("Expected error on directory traversal")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewCloudFSProvider(tmpDir)

    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Test write
    err := provider.WriteFile(ctx, "test.txt", []byte("cloud-hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Verify file written to correct tenant directory
    data, err := os.ReadFile(filepath.Join(tmpDir, "tenant-1", "test.txt"))
    if err != nil || string(data) != "cloud-hello" {
        t.Fatalf("File not written to correct path: %v", err)
    }

    // Test read
    readData, err := provider.ReadFile(ctx, "test.txt")
    if err != nil || string(readData) != "cloud-hello" {
        t.Fatalf("ReadFile failed: %v", err)
    }

    // Test no claims
    err = provider.WriteFile(context.Background(), "test.txt", []byte("fail"))
    if err == nil {
        t.Error("Expected error when no claims present")
    }
}

func TestHybridFSMCP(t *testing.T) {
    tmpDir := t.TempDir()
    provider := NewLocalFSProvider(tmpDir)
    mcp := NewHybridFSMCP(provider)
    ctx := context.Background()

    // Test ListTools
    tools := mcp.ListTools()
    if len(tools) != 3 {
        t.Errorf("Expected 3 tools, got %d", len(tools))
    }

    // Test CallTool
    _, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "mcp-hello"})
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }

    resMap, ok := res.(map[string]interface{})
    if !ok || resMap["content"] != "mcp-hello" {
        t.Errorf("CallTool read_file returned unexpected result")
    }
}
