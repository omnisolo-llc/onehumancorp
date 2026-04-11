package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestServerExecute(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewServer(provider)
	ctx := context.Background()

	// 1. Write file
	writeParams := []byte(`{"path": "test.txt", "content": "hello mcp"}`)
	res := server.Execute(ctx, "write_file", writeParams)
	if res.Status != "success" {
		t.Errorf("write_file failed: %s", string(res.ResultData))
	}

	// 2. Read file
	readParams := []byte(`{"path": "test.txt"}`)
	res = server.Execute(ctx, "read_file", readParams)
	if res.Status != "success" {
		t.Errorf("read_file failed: %s", string(res.ResultData))
	}
	var readData struct {
		Content string `json:"content"`
	}
	json.Unmarshal(res.ResultData, &readData)
	if readData.Content != "hello mcp" {
		t.Errorf("expected 'hello mcp', got '%s'", readData.Content)
	}

	// 3. List directory
	listParams := []byte(`{"path": "."}`)
	res = server.Execute(ctx, "list_directory", listParams)
	if res.Status != "success" {
		t.Errorf("list_directory failed: %s", string(res.ResultData))
	}
	var listData struct {
		Files []string `json:"files"`
	}
	json.Unmarshal(res.ResultData, &listData)
	if len(listData.Files) != 1 || listData.Files[0] != "test.txt" {
		t.Errorf("unexpected list_directory result: %v", listData.Files)
	}

	// 4. Unknown tool
	res = server.Execute(ctx, "unknown_tool", []byte(`{}`))
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool")
	}
}

func TestServerExecute_InvalidParams(t *testing.T) {
	provider, _ := NewLocalFSProvider(".")
	server := NewServer(provider)
	ctx := context.Background()

	invalidParams := []byte(`{invalid json}`)

	res := server.Execute(ctx, "read_file", invalidParams)
	if res.Status != "error" {
		t.Errorf("expected error for invalid params on read_file")
	}

	res = server.Execute(ctx, "write_file", invalidParams)
	if res.Status != "error" {
		t.Errorf("expected error for invalid params on write_file")
	}

	res = server.Execute(ctx, "list_directory", invalidParams)
	if res.Status != "error" {
		t.Errorf("expected error for invalid params on list_directory")
	}
}
