package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestNewFileSystemProvider(t *testing.T) {
	// Test Cloud Mode
	os.Setenv("OHC_MULTITENANT", "true")
	provider := NewFileSystemProvider()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider in multitenant mode")
	}

	// Test Local Mode
	os.Setenv("OHC_MULTITENANT", "false")
	provider = NewFileSystemProvider()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider in standalone mode")
	}
}

func TestHybridFSMCP_HandleRequest(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()
	mcpServer := NewHybridFSMCP(provider)

	// 1. Write file
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp content",
	}
	writeRes := mcpServer.HandleRequest(context.Background(), "write_file", writeArgs)
	if writeRes.Status != "success" {
		t.Errorf("write_file failed: %s", writeRes.ResultData)
	}

	var writeParsed map[string]bool
	json.Unmarshal(writeRes.ResultData, &writeParsed)
	if !writeParsed["success"] {
		t.Errorf("expected success=true in write_file")
	}

	// 2. Read file
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	readRes := mcpServer.HandleRequest(context.Background(), "read_file", readArgs)
	if readRes.Status != "success" {
		t.Errorf("read_file failed: %s", readRes.ResultData)
	}

	var readParsed map[string]string
	json.Unmarshal(readRes.ResultData, &readParsed)
	if readParsed["content"] != "mcp content" {
		t.Errorf("expected 'mcp content', got %q", readParsed["content"])
	}

	// 3. List directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	listRes := mcpServer.HandleRequest(context.Background(), "list_directory", listArgs)
	if listRes.Status != "success" {
		t.Errorf("list_directory failed: %s", listRes.ResultData)
	}

	var listParsed map[string][]string
	json.Unmarshal(listRes.ResultData, &listParsed)
	if len(listParsed["files"]) != 1 || listParsed["files"][0] != "mcp_test.txt" {
		t.Errorf("expected ['mcp_test.txt'], got %v", listParsed["files"])
	}

	// 4. Invalid tool
	invalidRes := mcpServer.HandleRequest(context.Background(), "invalid_tool", nil)
	if invalidRes.Status != "error" {
		t.Errorf("expected error for invalid_tool")
	}

	// 5. Missing args for read
	missingArgsRes := mcpServer.HandleRequest(context.Background(), "read_file", map[string]interface{}{})
	if missingArgsRes.Status != "error" {
		t.Errorf("expected error for missing args")
	}

	// 6. Missing args for write
	missingArgsWriteRes := mcpServer.HandleRequest(context.Background(), "write_file", map[string]interface{}{"path": "test"})
	if missingArgsWriteRes.Status != "error" {
		t.Errorf("expected error for missing write args")
	}

	// 7. Error passing down from provider (e.g. read non-existent)
	readErrArgs := map[string]interface{}{"path": "does_not_exist.txt"}
	readErrRes := mcpServer.HandleRequest(context.Background(), "read_file", readErrArgs)
	if readErrRes.Status != "error" {
		t.Errorf("expected error reading non existent file")
	}
}
