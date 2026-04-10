package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestFileSystemMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_server_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &LocalFSProvider{BaseDir: tempDir}
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// Test write_file
	writeInput := `{"path": "test.txt", "content": "hello mcp"}`
	res, err := server.HandleToolCall(ctx, "write_file", json.RawMessage(writeInput))
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s", res.Status)
	}

	// Test read_file
	readInput := `{"path": "test.txt"}`
	res, err = server.HandleToolCall(ctx, "read_file", json.RawMessage(readInput))
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	var readResult map[string]string
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%s'", readResult["content"])
	}

	// Test list_directory
	listInput := `{"path": "."}`
	res, err = server.HandleToolCall(ctx, "list_directory", json.RawMessage(listInput))
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	var listResult map[string][]string
	json.Unmarshal(res.ResultData, &listResult)
	if len(listResult["entries"]) != 1 || listResult["entries"][0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", listResult["entries"])
	}

	// Test unknown tool
	_, err = server.HandleToolCall(ctx, "unknown_tool", json.RawMessage(`{}`))
	if err == nil {
		t.Error("Expected error for unknown tool")
	}
}
