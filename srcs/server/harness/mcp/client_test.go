package mcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func createMockServerCmd(t *testing.T) string {
	// We create a temporary mock script that responds to MCP JSON-RPC requests
	script := `#!/bin/sh
while read -r line; do
  # Very basic mock logic for MCP
  if echo "$line" | grep -q '"method":"tools/list"'; then
    id=$(echo "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
    echo '{"jsonrpc":"2.0","id":"'$id'","result":{"tools":[{"name":"test_tool","description":"A test tool","inputSchema":{}}]}}'
  elif echo "$line" | grep -q '"method":"tools/call"'; then
    id=$(echo "$line" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
    if echo "$line" | grep -q 'error_tool'; then
       echo '{"jsonrpc":"2.0","id":"'$id'","error":{"code":-32603,"message":"Tool failed"}}'
    else
       echo '{"jsonrpc":"2.0","id":"'$id'","result":{"content":[{"type":"text","text":"success"}]}}'
    fi
  else
    # Echo back to keep the pipe open or handle other requests
    echo '{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"}}'
  fi
done
`
	tmpDir := t.TempDir()
	scriptPath := filepath.Join(tmpDir, "mock_server.sh")
	err := os.WriteFile(scriptPath, []byte(script), 0755)
	if err != nil {
		t.Fatalf("failed to create mock server script: %v", err)
	}
	return scriptPath
}

func TestConvertToMCPTool(t *testing.T) {
	internal := InternalTool{
		Name:        "test_tool",
		Description: "A simple test tool",
		Parameters: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"param1": map[string]interface{}{"type": "string"},
			},
		},
	}

	mcpTool := ConvertToMCPTool(internal)

	if mcpTool.Name != "test_tool" {
		t.Errorf("expected name test_tool, got %s", mcpTool.Name)
	}
	if mcpTool.Description != "A simple test tool" {
		t.Errorf("expected description 'A simple test tool', got %s", mcpTool.Description)
	}
	if mcpTool.InputSchema["type"] != "object" {
		t.Errorf("expected schema type object, got %v", mcpTool.InputSchema["type"])
	}
}

func TestClientManager_ConnectStdio(t *testing.T) {
	cmdPath := createMockServerCmd(t)

	manager := NewClientManager()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	config := ServerConfig{
		ID:      "test_server",
		Command: cmdPath,
		Args:    []string{},
		Env:     []string{},
	}

	err := manager.ConnectStdio(ctx, config)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}

	// Test connecting again with same ID
	err = manager.ConnectStdio(ctx, config)
	if err == nil {
		t.Errorf("expected error when connecting with existing ID")
	}
}

func TestClientManager_ListTools(t *testing.T) {
	cmdPath := createMockServerCmd(t)

	manager := NewClientManager()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	config := ServerConfig{
		ID:      "test_server",
		Command: cmdPath,
	}

	err := manager.ConnectStdio(ctx, config)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}

	tools, err := manager.ListTools(ctx, "test_server")
	if err != nil {
		t.Fatalf("failed to list tools: %v", err)
	}

	if len(tools) != 1 {
		t.Fatalf("expected 1 tool, got %d", len(tools))
	}

	if tools[0].Name != "test_tool" {
		t.Errorf("expected tool name test_tool, got %s", tools[0].Name)
	}

	// Test unknown server
	_, err = manager.ListTools(ctx, "unknown_server")
	if err == nil {
		t.Errorf("expected error listing tools on unknown server")
	}
}

func TestClientManager_CallTool(t *testing.T) {
	cmdPath := createMockServerCmd(t)

	manager := NewClientManager()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	config := ServerConfig{
		ID:      "test_server",
		Command: cmdPath,
	}

	err := manager.ConnectStdio(ctx, config)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}

	// Test successful call
	args := map[string]interface{}{"param": "value"}
	result, err := manager.CallTool(ctx, "test_server", "test_tool", args)
	if err != nil {
		t.Fatalf("failed to call tool: %v", err)
	}

	if len(result.Content) != 1 {
		t.Fatalf("expected 1 content item, got %d", len(result.Content))
	}

	contentMap, ok := result.Content[0].(map[string]interface{})
	if !ok {
		t.Fatalf("expected content item to be map, got %T", result.Content[0])
	}

	if contentMap["text"] != "success" {
		t.Errorf("expected text success, got %v", contentMap["text"])
	}

	// Test error call
	_, err = manager.CallTool(ctx, "test_server", "error_tool", args)
	if err == nil {
		t.Errorf("expected error when calling error_tool")
	}

	// Test unknown server
	_, err = manager.CallTool(ctx, "unknown_server", "test_tool", args)
	if err == nil {
		t.Errorf("expected error calling tool on unknown server")
	}
}

func TestClientManager_Disconnect(t *testing.T) {
	cmdPath := createMockServerCmd(t)

	manager := NewClientManager()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	config := ServerConfig{
		ID:      "test_server",
		Command: cmdPath,
	}

	err := manager.ConnectStdio(ctx, config)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}

	// Wait a moment for server to start
	time.Sleep(100 * time.Millisecond)

	manager.Disconnect("test_server")

	// Verify server is removed
	manager.mu.RLock()
	_, ok := manager.servers["test_server"]
	manager.mu.RUnlock()

	if ok {
		t.Errorf("expected server to be removed from map")
	}

	// Disconnecting an unknown server should not panic
	manager.Disconnect("unknown_server")
}

func TestClientManager_ContextCancellation(t *testing.T) {
	// Script that hangs on call to simulate a timeout
	script := `#!/bin/sh
while read -r line; do
  if echo "$line" | grep -q '"method":"tools/call"'; then
    sleep 5
  fi
done
`
	tmpDir := t.TempDir()
	cmdPath := filepath.Join(tmpDir, "mock_server_hang.sh")
	err := os.WriteFile(cmdPath, []byte(script), 0755)
	if err != nil {
		t.Fatalf("failed to create mock server script: %v", err)
	}

	manager := NewClientManager()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	config := ServerConfig{
		ID:      "test_server",
		Command: cmdPath,
	}

	err = manager.ConnectStdio(ctx, config)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}

	// Create a call context that we will cancel quickly
	callCtx, callCancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer callCancel()

	_, err = manager.CallTool(callCtx, "test_server", "test_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error due to context cancellation")
	} else if err != context.DeadlineExceeded {
		t.Errorf("expected context deadline exceeded, got: %v", err)
	}
}

// Test JSONRPC serialization directly
func TestJSONRPCMessageSerialization(t *testing.T) {
	id := "123"
	req := JSONRPCMessage{
		JSONRPC: "2.0",
		ID:      &id,
		Method:  "tools/list",
	}

	data, err := json.Marshal(req)
	if err != nil {
		t.Fatalf("failed to marshal req: %v", err)
	}

	str := string(data)
	if str != `{"jsonrpc":"2.0","id":"123","method":"tools/list"}` {
		t.Errorf("unexpected marshaled json: %s", str)
	}
}
