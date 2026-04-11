package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"reflect"
	"testing"
)

func TestFileSystemMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_server_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	server := NewFileSystemMCP(provider)
	ctx := context.Background()

	// 1. Write File
	writeArgs := map[string]interface{}{
		"path": "mcp_test.txt",
		"data": []byte("mcp data"),
	}
	writeArgsBytes, _ := json.Marshal(writeArgs)

	res, err := server.HandleRequest(ctx, "write_file", writeArgsBytes)
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	if res.ToolID != "write_file" || res.Status != "success" {
		t.Errorf("Unexpected write_file result: %+v", res)
	}

	// 2. Read File
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	readArgsBytes, _ := json.Marshal(readArgs)

	res, err = server.HandleRequest(ctx, "read_file", readArgsBytes)
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}

	var readResult map[string][]byte
	if err := json.Unmarshal(res.ResultData, &readResult); err != nil {
		t.Fatalf("Failed to parse read_file result: %v", err)
	}
	if string(readResult["data"]) != "mcp data" {
		t.Errorf("Expected 'mcp data', got '%s'", string(readResult["data"]))
	}

	// 3. List Directory
	listArgs := map[string]interface{}{
		"path": "",
	}
	listArgsBytes, _ := json.Marshal(listArgs)

	res, err = server.HandleRequest(ctx, "list_directory", listArgsBytes)
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}

	var listResult map[string][]string
	if err := json.Unmarshal(res.ResultData, &listResult); err != nil {
		t.Fatalf("Failed to parse list_directory result: %v", err)
	}
	if !reflect.DeepEqual(listResult["files"], []string{"mcp_test.txt"}) {
		t.Errorf("Expected ['mcp_test.txt'], got %v", listResult["files"])
	}

	// 4. Search Files
	searchArgs := map[string]interface{}{
		"path":    "",
		"pattern": "mcp",
	}
	searchArgsBytes, _ := json.Marshal(searchArgs)

	res, err = server.HandleRequest(ctx, "search_files", searchArgsBytes)
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}

	var searchResult map[string][]string
	if err := json.Unmarshal(res.ResultData, &searchResult); err != nil {
		t.Fatalf("Failed to parse search_files result: %v", err)
	}
	if !reflect.DeepEqual(searchResult["files"], []string{"mcp_test.txt"}) {
		t.Errorf("Expected ['mcp_test.txt'], got %v", searchResult["files"])
	}

	// 5. Unknown Tool
	_, err = server.HandleRequest(ctx, "unknown_tool", []byte(`{}`))
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}
