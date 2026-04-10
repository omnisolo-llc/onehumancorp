package hybridfsmcp

import (
    "context"
    "encoding/base64"
    "os"
    "testing"
)

func TestHybridFSProxyMCP(t *testing.T) {
    baseDir, err := os.MkdirTemp("", "mcptest")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(baseDir)

    provider := NewLocalFSProvider(baseDir)
    mcp := NewHybridFSProxyMCP(provider)
    ctx := context.Background()

    // Test list tools
    tools := mcp.ListTools()
    if len(tools) != 3 {
        t.Errorf("expected 3 tools, got %d", len(tools))
    }

    // Test write
    contentB64 := base64.StdEncoding.EncodeToString([]byte("mcp hello"))
    _, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
        "path": "test_mcp.txt",
        "content_base64": contentB64,
    })
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    // Test read
    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
        "path": "test_mcp.txt",
    })
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }
    resMap := res.(map[string]interface{})
    if resMap["content_base64"] != contentB64 {
        t.Errorf("expected %s, got %s", contentB64, resMap["content_base64"])
    }

    // Test list
    resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
        "path": "",
    })
    if err != nil {
        t.Fatalf("CallTool list_directory failed: %v", err)
    }
    resListMap := resList.(map[string]interface{})
    files := resListMap["files"].([]string)
    if len(files) != 1 || files[0] != "test_mcp.txt" {
        t.Errorf("unexpected files: %v", files)
    }
}
