package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestMCPServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_MULTITENANT", "false")
	os.Setenv("OHC_WORKSPACE_DIR", tmpDir)
	defer os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	server, err := NewServer()
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	ctx := context.Background()

	writeArgs := WriteFileArgs{Path: "mcp_test.txt", Data: "mcp data"}
	rawWriteArgs, _ := json.Marshal(writeArgs)
	writeRes := server.ExecuteTool(ctx, "write_file", rawWriteArgs)
	if writeRes.Status != "success" {
		t.Errorf("expected success writing file, got %v", writeRes.Status)
	}

	readArgs := ReadFileArgs{Path: "mcp_test.txt"}
	rawReadArgs, _ := json.Marshal(readArgs)
	readRes := server.ExecuteTool(ctx, "read_file", rawReadArgs)
	if readRes.Status != "success" {
		t.Errorf("expected success reading file, got %v", readRes.Status)
	}
	if string(readRes.ResultData) != "mcp data" {
		t.Errorf("expected result 'mcp data', got %s", readRes.ResultData)
	}

	listArgs := ListDirArgs{Path: "."}
	rawListArgs, _ := json.Marshal(listArgs)
	listRes := server.ExecuteTool(ctx, "list_directory", rawListArgs)
	if listRes.Status != "success" {
		t.Errorf("expected success listing dir, got %v", listRes.Status)
	}
}
