package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestMCPServer(t *testing.T) {
	// Set up local mode
	os.Setenv("OHC_MULTITENANT", "false")
	tmpDir := t.TempDir()
	os.Setenv("OHC_STANDALONE_WORKSPACE", tmpDir)

	server, err := NewFileSystemMCPServer()
	if err != nil {
		t.Fatalf("Failed to create MCP server: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	writeInput := WriteFileInput{Path: "mcp_test.txt", Content: "mcp content"}
	writeJSON, _ := json.Marshal(writeInput)
	res, err := server.ExecuteTool(ctx, "write_file", writeJSON)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s", res.Status)
	}

	// Test ReadFile
	readInput := ReadFileInput{Path: "mcp_test.txt"}
	readJSON, _ := json.Marshal(readInput)
	res, err = server.ExecuteTool(ctx, "read_file", readJSON)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}

	var readRes map[string]string
	if err := json.Unmarshal(res.ResultData, &readRes); err != nil {
		t.Fatalf("Failed to unmarshal result data: %v", err)
	}
	if readRes["content"] != "mcp content" {
		t.Errorf("Expected 'mcp content', got '%s'", readRes["content"])
	}

	// Test ListDirectory
	listInput := ListDirectoryInput{Path: "."}
	listJSON, _ := json.Marshal(listInput)
	res, err = server.ExecuteTool(ctx, "list_directory", listJSON)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}

	var listRes map[string][]string
	if err := json.Unmarshal(res.ResultData, &listRes); err != nil {
		t.Fatalf("Failed to unmarshal list result: %v", err)
	}

	found := false
	for _, f := range listRes["files"] {
		if f == "mcp_test.txt" {
			found = true
			break
		}
	}
	if !found {
		t.Error("Expected mcp_test.txt to be in listed directory")
	}

	// Test Bundle
	bundle, err := NewHybridFSBundle()
	if err != nil {
		t.Fatalf("Failed to create bundle: %v", err)
	}
	tools := bundle.GetTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}
}
