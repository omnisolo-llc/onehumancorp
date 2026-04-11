package hybridfsmcp

import (
    "context"
    "encoding/json"
    "os"
    "path/filepath"
    "testing"
)

func TestLocalFSProvider(t *testing.T) {
    tempDir := t.TempDir()
    provider := NewLocalFSProvider(tempDir)
    ctx := context.Background()

    // Test WriteFile
    err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Test ReadFile
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello" {
        t.Fatalf("expected 'hello', got '%s'", string(data))
    }

    // Test Path Escape (should fail)
    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
    if err == nil {
        t.Fatalf("expected path escape to fail")
    }

    // Test ListDir
    entries, err := provider.ListDir(ctx, ".")
    if err != nil {
       t.Fatalf("ListDir failed: %v", err)
    }
    if len(entries) != 1 || entries[0].Name() != "test.txt" {
       t.Fatalf("ListDir unexpected output")
    }
}

func TestCloudFSProvider(t *testing.T) {
    tempDir := t.TempDir()
    provider := NewCloudFSProvider(tempDir, "tenant1")
    ctx := context.Background()

    // Test WriteFile
    err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
    if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
    }

    // Verify path is actually under tenant
    _, err = os.Stat(filepath.Join(tempDir, "tenant1", "test.txt"))
    if err != nil {
        t.Fatalf("File was not created in tenant dir: %v", err)
    }

    // Test ReadFile
    data, err := provider.ReadFile(ctx, "test.txt")
    if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
    }
    if string(data) != "hello cloud" {
        t.Fatalf("expected 'hello cloud', got '%s'", string(data))
    }

    // Test Path Escape (should fail)
    err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
    if err == nil {
        t.Fatalf("expected path escape to fail")
    }
}

func TestHybridFSMCP(t *testing.T) {
   tempDir := t.TempDir()
   server := NewHybridFSMCP(false, tempDir, "")
   ctx := context.Background()

   args1, _ := json.Marshal(WriteFileArgs{Path: "mcp.txt", Data: []byte("mcp")})
   _, err := server.ExecuteTool(ctx, "write_file", args1)
   if err != nil {
        t.Fatalf("WriteFile failed: %v", err)
   }

   args2, _ := json.Marshal(ReadFileArgs{Path: "mcp.txt"})
   res, err := server.ExecuteTool(ctx, "read_file", args2)
   if err != nil {
        t.Fatalf("ReadFile failed: %v", err)
   }
   if res.(string) != "mcp" {
       t.Fatalf("Expected 'mcp' got %v", res)
   }

   args3, _ := json.Marshal(ListDirArgs{Path: "."})
   resList, err := server.ExecuteTool(ctx, "list_directory", args3)
   if err != nil {
        t.Fatalf("ListDir failed: %v", err)
   }
   list := resList.([]string)
   if len(list) != 1 || list[0] != "mcp.txt" {
       t.Fatalf("ListDir unexpected output")
   }
}
