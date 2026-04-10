package hybridfsmcp

import (
    "context"
    "os"
    "path/filepath"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
    tempDir := t.TempDir()
    provider, err := NewLocalFSProvider(tempDir)
    if err != nil {
        t.Fatalf("Failed to create provider: %v", err)
    }

    ctx := context.Background()

    // Test WriteFile
    err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Test ReadFile
    content, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(content) != "hello" {
        t.Errorf("Expected 'hello', got '%s'", string(content))
    }

    // Test Path Traversal
    _, err = provider.ReadFile(ctx, "../outside.txt")
    if err == nil {
        t.Errorf("Expected path traversal error")
    }

    // Test Absolute Path rejection
    _, err = provider.ReadFile(ctx, "/etc/passwd")
    if err == nil {
        t.Errorf("Expected absolute path error")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tempDir := t.TempDir()
    provider, err := NewCloudFSProvider(tempDir)
    if err != nil {
        t.Fatalf("Failed to create provider: %v", err)
    }

    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
        OrganizationID: "tenant1",
    })

    // Test WriteFile
    err = provider.WriteFile(ctx, "data.txt", []byte("cloud"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Verify isolation
    tenantPath := filepath.Join(tempDir, "tenant1", "data.txt")
    if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
        t.Errorf("File was not isolated to tenant directory")
    }

    // Test unauthorized (no claims)
    err = provider.WriteFile(context.Background(), "data.txt", []byte("fail"))
    if err == nil {
        t.Errorf("Expected authorization error")
    }
}

func TestHybridFSMCP(t *testing.T) {
    tempDir := t.TempDir()
    mcp, err := NewHybridFSMCP(true, tempDir)
    if err != nil {
        t.Fatalf("Failed to create MCP: %v", err)
    }

    ctx := context.Background()

    // Write
    _, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
        "path":    "file.txt",
        "content": "mcp test",
    })
    if err != nil {
        t.Fatalf("CallTool write_file failed: %v", err)
    }

    // Read
    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
        "path": "file.txt",
    })
    if err != nil {
        t.Fatalf("CallTool read_file failed: %v", err)
    }

    contentMap, ok := res.(map[string]interface{})
    if !ok {
        t.Fatalf("Unexpected result type")
    }

    if contentMap["content"] != "mcp test" {
        t.Errorf("Expected 'mcp test', got '%v'", contentMap["content"])
    }
}
