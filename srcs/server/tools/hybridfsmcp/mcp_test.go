package hybridfsmcp

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

func TestMCPCloudMode(t *testing.T) {
    dir := t.TempDir()
    provider, _ := NewCloudFSProvider(dir)
    mcp := NewHybridFSMCPServer(provider)

    tools := mcp.ListTools()
    if len(tools) != 3 {
        t.Fatalf("Expected 3 tools")
    }

    ctx := context.Background()

    // Should fail without claims in cloud mode
    _, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "a.txt", "content": "hi"})
    if err == nil {
        t.Fatalf("Expected error missing claims")
    }

    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
        OrganizationID: "org-1",
    })

    res, err := mcp.CallTool(ctxWithClaims, "write_file", map[string]interface{}{"path": "a.txt", "content": "hi"})
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    res, err = mcp.CallTool(ctxWithClaims, "read_file", map[string]interface{}{"path": "a.txt"})
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }
    r := res.(map[string]interface{})
    if r["content"] != "hi" {
        t.Fatalf("Unexpected content")
    }

    res, err = mcp.CallTool(ctxWithClaims, "list_directory", map[string]interface{}{"path": ""})
    if err != nil {
        t.Fatalf("CallTool list_directory failed: %v", err)
    }
    r = res.(map[string]interface{})
    entries := r["entries"].([]map[string]interface{})
    if len(entries) != 1 || entries[0]["name"] != "a.txt" {
        t.Fatalf("Unexpected entries")
    }
}

func TestMCPLocalMode(t *testing.T) {
    dir := t.TempDir()
    provider, _ := NewLocalFSProvider(dir)
    mcp := NewHybridFSMCPServer(provider)

    ctx := context.Background()

    _, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "b.txt", "content": "local"})
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "b.txt"})
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }
    r := res.(map[string]interface{})
    if r["content"] != "local" {
        t.Fatalf("Unexpected content")
    }
}
