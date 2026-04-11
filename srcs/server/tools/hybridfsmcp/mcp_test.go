package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcpServer, err := NewHybridFSMCP()
	if err != nil {
		t.Fatalf("failed to create MCP server: %v", err)
	}

	ctx := context.Background()

	// 1. write_file
	writeRes, err := mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	writeResMap := writeRes.(map[string]interface{})
	if writeResMap["status"] != "success" {
		t.Errorf("expected success, got %v", writeResMap["status"])
	}

	// 2. read_file
	readRes, err := mcpServer.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	readResMap := readRes.(map[string]interface{})
	if readResMap["content"] != "world" {
		t.Errorf("expected 'world', got %v", readResMap["content"])
	}

	// 3. list_directory
	listRes, err := mcpServer.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	listResMap := listRes.(map[string]interface{})
	entries := listResMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("expected ['hello.txt'], got %v", entries)
	}

	// 4. Test missing args
	_, err = mcpServer.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for missing path")
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

	mcpServer, err := NewHybridFSMCP()
	if err != nil {
		t.Fatalf("failed to create MCP server: %v", err)
	}

	ctx := context.Background()

	// List tools
	listReq := map[string]interface{}{
		"jsonrpc": "2.0",
		"id":      1,
		"method":  "tools/list",
	}
	reqBytes, _ := json.Marshal(listReq)

	resBytes, err := mcpServer.HandleRequest(ctx, reqBytes)
	if err != nil {
		t.Fatalf("HandleRequest failed: %v", err)
	}

	var res map[string]interface{}
	json.Unmarshal(resBytes, &res)

	if res["jsonrpc"] != "2.0" {
		t.Errorf("expected jsonrpc 2.0, got %v", res["jsonrpc"])
	}

	resultMap, ok := res["result"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected result object")
	}

	if tools, ok := resultMap["tools"].([]interface{}); !ok || len(tools) != 3 {
		t.Errorf("expected 3 tools, got %v", tools)
	}

	// Call tool
	callReq := map[string]interface{}{
		"jsonrpc": "2.0",
		"id":      2,
		"method":  "tools/call",
		"params": map[string]interface{}{
			"name": "write_file",
			"arguments": map[string]interface{}{
				"path": "test.txt",
				"content": "abc",
			},
		},
	}
	reqBytes, _ = json.Marshal(callReq)
	resBytes, err = mcpServer.HandleRequest(ctx, reqBytes)
	if err != nil {
		t.Fatalf("HandleRequest failed: %v", err)
	}
	json.Unmarshal(resBytes, &res)

	resultMap, ok = res["result"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected result object")
	}

	content := resultMap["content"].([]interface{})[0].(map[string]interface{})["text"].(string)
	var contentMap map[string]interface{}
	json.Unmarshal([]byte(content), &contentMap)

	if contentMap["status"] != "success" {
		t.Errorf("expected success, got %v", contentMap["status"])
	}
}