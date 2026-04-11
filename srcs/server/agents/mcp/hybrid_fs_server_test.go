package mcp

import (
	"context"
	"encoding/json"
	"os"
	"reflect"
	"testing"
)

func TestFileSystemMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "fs_mcp_server_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// Test Write File
	writePayload := []byte(`{"path": "test.txt", "content": "hello mcp"}`)
	res := server.ExecuteTool(ctx, "write_file", writePayload)
	if res.Status != "success" {
		t.Errorf("Expected success, got %v: %s", res.Status, string(res.ResultData))
	}

	// Test Read File
	readPayload := []byte(`{"path": "test.txt"}`)
	res = server.ExecuteTool(ctx, "read_file", readPayload)
	if res.Status != "success" {
		t.Errorf("Expected success, got %v: %s", res.Status, string(res.ResultData))
	}
	if string(res.ResultData) != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got %s", string(res.ResultData))
	}

	// Test List Directory
	listPayload := []byte(`{"path": "."}`)
	res = server.ExecuteTool(ctx, "list_directory", listPayload)
	if res.Status != "success" {
		t.Errorf("Expected success, got %v: %s", res.Status, string(res.ResultData))
	}
	var listResult map[string][]string
	if err := json.Unmarshal(res.ResultData, &listResult); err != nil {
		t.Fatalf("Failed to unmarshal result: %v", err)
	}
	if !reflect.DeepEqual(listResult["files"], []string{"test.txt"}) {
		t.Errorf("Expected ['test.txt'], got %v", listResult["files"])
	}

	// Test Search Files
	server.ExecuteTool(ctx, "write_file", []byte(`{"path": "test2.log", "content": "log data"}`))
	searchPayload := []byte(`{"path": ".", "pattern": ".log"}`)
	res = server.ExecuteTool(ctx, "search_files", searchPayload)
	if res.Status != "success" {
		t.Errorf("Expected success, got %v: %s", res.Status, string(res.ResultData))
	}
	var searchResult map[string][]string
	if err := json.Unmarshal(res.ResultData, &searchResult); err != nil {
		t.Fatalf("Failed to unmarshal result: %v", err)
	}
	if !reflect.DeepEqual(searchResult["files"], []string{"test2.log"}) {
		t.Errorf("Expected ['test2.log'], got %v", searchResult["files"])
	}

	// Test Unknown Tool
	res = server.ExecuteTool(ctx, "unknown_tool", []byte(`{}`))
	if res.Status != "error" {
		t.Errorf("Expected error for unknown tool, got %v", res.Status)
	}
}
