package hybridfsmcp

import (
	"context"
	"encoding/json"
	"reflect"
	"testing"
)

func TestServer(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewServer(provider)
	ctx := context.Background()

	// Test ListTools
	tools, err := server.ListTools(ctx)
	if err != nil {
		t.Errorf("ListTools failed: %v", err)
	}
	expectedTools := []string{"read_file", "write_file", "list_directory", "search_files"}
	if !reflect.DeepEqual(tools, expectedTools) {
		t.Errorf("expected %v, got %v", expectedTools, tools)
	}

	// Test write_file
	writeArgs := map[string]string{
		"path":    "test.txt",
		"content": "hello mcp",
	}
	writeArgsJSON, _ := json.Marshal(writeArgs)
	writeRes, err := server.CallTool(ctx, CallToolRequest{Name: "write_file", Arguments: writeArgsJSON})
	if err != nil {
		t.Errorf("CallTool(write_file) failed: %v", err)
	}
	if writeRes.Status != "success" {
		t.Errorf("expected success, got %s", writeRes.Status)
	}

	// Test read_file
	readArgs := map[string]string{
		"path": "test.txt",
	}
	readArgsJSON, _ := json.Marshal(readArgs)
	readRes, err := server.CallTool(ctx, CallToolRequest{Name: "read_file", Arguments: readArgsJSON})
	if err != nil {
		t.Errorf("CallTool(read_file) failed: %v", err)
	}
	if readRes.Status != "success" {
		t.Errorf("expected success, got %s", readRes.Status)
	}
	var readData map[string]string
	json.Unmarshal(readRes.ResultData, &readData)
	if readData["content"] != "hello mcp" {
		t.Errorf("expected 'hello mcp', got %s", readData["content"])
	}

	// Test list_directory
	listArgs := map[string]string{
		"path": ".",
	}
	listArgsJSON, _ := json.Marshal(listArgs)
	listRes, err := server.CallTool(ctx, CallToolRequest{Name: "list_directory", Arguments: listArgsJSON})
	if err != nil {
		t.Errorf("CallTool(list_directory) failed: %v", err)
	}
	if listRes.Status != "success" {
		t.Errorf("expected success, got %s", listRes.Status)
	}
	var listData map[string][]string
	json.Unmarshal(listRes.ResultData, &listData)
	if len(listData["entries"]) != 1 || listData["entries"][0] != "test.txt" {
		t.Errorf("expected [test.txt], got %v", listData["entries"])
	}

	// Test search_files
	searchArgs := map[string]string{
		"dir":     ".",
		"pattern": "*.txt",
	}
	searchArgsJSON, _ := json.Marshal(searchArgs)
	searchRes, err := server.CallTool(ctx, CallToolRequest{Name: "search_files", Arguments: searchArgsJSON})
	if err != nil {
		t.Errorf("CallTool(search_files) failed: %v", err)
	}
	if searchRes.Status != "success" {
		t.Errorf("expected success, got %s", searchRes.Status)
	}
	var searchData map[string][]string
	json.Unmarshal(searchRes.ResultData, &searchData)
	if len(searchData["matches"]) != 1 || searchData["matches"][0] != "test.txt" {
		t.Errorf("expected [test.txt], got %v", searchData["matches"])
	}

	// Test unknown tool
	unknownRes, err := server.CallTool(ctx, CallToolRequest{Name: "unknown", Arguments: []byte("{}")})
	if err == nil {
		t.Errorf("expected error for unknown tool, got nil")
	}
	if unknownRes != nil {
		t.Errorf("expected nil result for unknown tool")
	}
}
