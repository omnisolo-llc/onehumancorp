package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestFileSystemMCPServer_ExecuteTool(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// 1. Test WriteFile
	writeArgs, _ := json.Marshal(WriteFileArgs{
		Path: "test-mcp.txt",
		Data: "hello mcp",
	})
	writeRes := server.ExecuteTool(ctx, MCPRequest{
		ToolID: ToolWriteFile,
		Args:   writeArgs,
	})
	if writeRes.Status != "success" {
		t.Fatalf("WriteFile failed: %v", writeRes.Error)
	}

	// Verify it wrote properly
	data, err := os.ReadFile(filepath.Join(tempDir, "test-mcp.txt"))
	if err != nil || string(data) != "hello mcp" {
		t.Fatalf("File not written correctly: %v", err)
	}

	// 2. Test ReadFile
	readArgs, _ := json.Marshal(ReadFileArgs{
		Path: "test-mcp.txt",
	})
	readRes := server.ExecuteTool(ctx, MCPRequest{
		ToolID: ToolReadFile,
		Args:   readArgs,
	})
	if readRes.Status != "success" {
		t.Fatalf("ReadFile failed: %v", readRes.Error)
	}
	if readRes.Result.(string) != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%v'", readRes.Result)
	}

	// 3. Test ListDir
	listArgs, _ := json.Marshal(ListDirArgs{
		Path: ".",
	})
	listRes := server.ExecuteTool(ctx, MCPRequest{
		ToolID: ToolListDir,
		Args:   listArgs,
	})
	if listRes.Status != "success" {
		t.Fatalf("ListDir failed: %v", listRes.Error)
	}
	files := listRes.Result.([]string)
	if len(files) != 1 || files[0] != "test-mcp.txt" {
		t.Errorf("Expected ['test-mcp.txt'], got %v", files)
	}

	// 4. Test unknown tool
	unknownRes := server.ExecuteTool(ctx, MCPRequest{
		ToolID: "unknown_tool",
		Args:   []byte(`{}`),
	})
	if unknownRes.Status != "error" {
		t.Errorf("Expected error for unknown tool")
	}
}
