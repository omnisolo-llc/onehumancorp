package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestMCPServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// write
	writeReq := ToolRequest{
		ToolID: "write_file",
		Params: []byte(`{"path": "test.txt", "data": "hello mcp"}`),
	}
	res1 := server.HandleTool(ctx, writeReq)
	if res1.Status != "success" {
		t.Errorf("expected success, got %s, data: %s", res1.Status, string(res1.ResultData))
	}

	// read
	readReq := ToolRequest{
		ToolID: "read_file",
		Params: []byte(`{"path": "test.txt"}`),
	}
	res2 := server.HandleTool(ctx, readReq)
	if res2.Status != "success" {
		t.Errorf("expected success, got %s", res2.Status)
	}
	if string(res2.ResultData) != "hello mcp" {
		t.Errorf("expected 'hello mcp', got %s", string(res2.ResultData))
	}

	// list
	listReq := ToolRequest{
		ToolID: "list_directory",
		Params: []byte(`{"path": "."}`),
	}
	res3 := server.HandleTool(ctx, listReq)
	if res3.Status != "success" {
		t.Errorf("expected success, got %s", res3.Status)
	}
	var entries []string
	json.Unmarshal(res3.ResultData, &entries)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// search
	searchReq := ToolRequest{
		ToolID: "search_files",
		Params: []byte(`{"directory": ".", "pattern": "*.txt"}`),
	}
	res4 := server.HandleTool(ctx, searchReq)
	if res4.Status != "success" {
		t.Errorf("expected success, got %s", res4.Status)
	}
	var matches []string
	json.Unmarshal(res4.ResultData, &matches)
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", matches)
	}

	// unknown tool
	unknownReq := ToolRequest{
		ToolID: "unknown_tool",
		Params: []byte(`{}`),
	}
	res5 := server.HandleTool(ctx, unknownReq)
	if res5.Status != "error" {
		t.Errorf("expected error, got %s", res5.Status)
	}
}
