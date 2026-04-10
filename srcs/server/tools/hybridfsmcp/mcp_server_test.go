package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcpserver")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewMCPServer(provider)
	ctx := context.Background()

	// Test write_file
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Content: "mcp test"})
	res, err := server.ExecuteTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Errorf("ExecuteTool write_file failed: %v", err)
	}
	if res.(string) != "File written successfully" {
		t.Errorf("Unexpected write result: %v", res)
	}

	// Test read_file
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err = server.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Errorf("ExecuteTool read_file failed: %v", err)
	}
	if res.(string) != "mcp test" {
		t.Errorf("Expected 'mcp test', got %v", res)
	}

	// Test list_directory
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = server.ExecuteTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Errorf("ExecuteTool list_directory failed: %v", err)
	}
	names := res.([]string)
	if len(names) != 1 || names[0] != "test.txt" {
		t.Errorf("Unexpected list_directory result: %v", res)
	}
}

func TestMCPServer_Errors(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "mcpservererr")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewMCPServer(provider)
	ctx := context.Background()

	// Test invalid write JSON
	_, err := server.ExecuteTool(ctx, "write_file", []byte("invalid json"))
	if err == nil {
		t.Errorf("Expected error for invalid write JSON, got nil")
	}

	// Test write_file failure from provider (e.g., path traversal)
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "../test.txt", Content: "mcp test"})
	_, err = server.ExecuteTool(ctx, "write_file", writeArgs)
	if err == nil {
		t.Errorf("Expected error for provider write failure, got nil")
	}

	// Test invalid read JSON
	_, err = server.ExecuteTool(ctx, "read_file", []byte("invalid json"))
	if err == nil {
		t.Errorf("Expected error for invalid read JSON, got nil")
	}

	// Test read_file failure from provider
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "nonexistent.txt"})
	_, err = server.ExecuteTool(ctx, "read_file", readArgs)
	if err == nil {
		t.Errorf("Expected error for provider read failure, got nil")
	}

	// Test invalid list JSON
	_, err = server.ExecuteTool(ctx, "list_directory", []byte("invalid json"))
	if err == nil {
		t.Errorf("Expected error for invalid list JSON, got nil")
	}

	// Test list_directory failure from provider
	listArgs, _ := json.Marshal(ListDirArgs{Path: "nonexistent_dir"})
	_, err = server.ExecuteTool(ctx, "list_directory", listArgs)
	if err == nil {
		t.Errorf("Expected error for provider list failure, got nil")
	}

	// Test unknown tool
	_, err = server.ExecuteTool(ctx, "unknown_tool", []byte("{}"))
	if err == nil {
		t.Errorf("Expected error for unknown tool, got nil")
	}
}
