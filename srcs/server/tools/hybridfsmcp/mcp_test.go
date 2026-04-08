package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestHybridFSServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// Test ListTools
	tools, err := server.ListTools(ctx)
	if err != nil {
		t.Fatal(err)
	}
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Test write_file
	writeReq := CallRequest{
		Name: "write_file",
		Arguments: map[string]string{
			"path":    "test.txt",
			"content": "mcp content",
		},
	}
	writeJSON, _ := json.Marshal(writeReq)

	resJSON, err := server.CallTool(ctx, writeJSON)
	if err != nil {
		t.Fatal(err)
	}

	var writeRes CallResponse
	json.Unmarshal(resJSON, &writeRes)
	if writeRes.Error != "" {
		t.Errorf("write_file error: %s", writeRes.Error)
	}
	if writeRes.Result != "success" {
		t.Errorf("Expected success, got %s", writeRes.Result)
	}

	// Test read_file
	readReq := CallRequest{
		Name: "read_file",
		Arguments: map[string]string{
			"path": "test.txt",
		},
	}
	readJSON, _ := json.Marshal(readReq)

	resJSON, err = server.CallTool(ctx, readJSON)
	if err != nil {
		t.Fatal(err)
	}

	var readRes CallResponse
	json.Unmarshal(resJSON, &readRes)
	if readRes.Error != "" {
		t.Errorf("read_file error: %s", readRes.Error)
	}
	if readRes.Result != "mcp content" {
		t.Errorf("Expected 'mcp content', got %s", readRes.Result)
	}

	// Test list_directory
	listReq := CallRequest{
		Name: "list_directory",
		Arguments: map[string]string{
			"path": ".",
		},
	}
	listJSON, _ := json.Marshal(listReq)

	resJSON, err = server.CallTool(ctx, listJSON)
	if err != nil {
		t.Fatal(err)
	}

	var listRes CallResponse
	json.Unmarshal(resJSON, &listRes)
	if listRes.Error != "" {
		t.Errorf("list_directory error: %s", listRes.Error)
	}
	if listRes.Result != "test.txt" {
		t.Errorf("Expected 'test.txt', got %s", listRes.Result)
	}

	// Test unknown tool
	unknownReq := CallRequest{
		Name: "unknown_tool",
	}
	unknownJSON, _ := json.Marshal(unknownReq)

	resJSON, err = server.CallTool(ctx, unknownJSON)
	if err != nil {
		t.Fatal(err)
	}

	var unknownRes CallResponse
	json.Unmarshal(resJSON, &unknownRes)
	if unknownRes.Error == "" {
		t.Error("Expected error for unknown tool")
	}
}
