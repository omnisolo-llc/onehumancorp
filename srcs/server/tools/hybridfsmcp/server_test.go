package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestServer_ToolsList(t *testing.T) {
	provider := NewLocalFSProvider(os.TempDir())
	server := NewServer(provider)

	req := JSONRPCRequest{
		JSONRPC: "2.0",
		ID:      1,
		Method:  "tools/list",
	}
	reqBody, _ := json.Marshal(req)

	respBody := server.HandleRequest(context.Background(), reqBody)

	var resp JSONRPCResponse
	if err := json.Unmarshal(respBody, &resp); err != nil {
		t.Fatalf("Failed to parse response: %v", err)
	}

	if resp.Error != nil {
		t.Fatalf("Unexpected error: %v", resp.Error)
	}

	resultMap, ok := resp.Result.(map[string]interface{})
	if !ok {
		t.Fatalf("Unexpected result type")
	}

	tools, ok := resultMap["tools"].([]interface{})
	if !ok || len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %v", len(tools))
	}
}

func TestServer_ToolsCall(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "fs_server_test")
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	// Write file
	writeParams := `{"name": "write_file", "arguments": {"path": "test.txt", "content": "world"}}`
	req := JSONRPCRequest{
		JSONRPC: "2.0",
		ID:      2,
		Method:  "tools/call",
		Params:  json.RawMessage(writeParams),
	}
	reqBody, _ := json.Marshal(req)
	respBody := server.HandleRequest(ctx, reqBody)

	var resp JSONRPCResponse
	json.Unmarshal(respBody, &resp)
	if resp.Error != nil {
		t.Fatalf("Unexpected error: %v", resp.Error)
	}

	// Read file
	readParams := `{"name": "read_file", "arguments": {"path": "test.txt"}}`
	req.Params = json.RawMessage(readParams)
	reqBody, _ = json.Marshal(req)
	respBody = server.HandleRequest(ctx, reqBody)

	json.Unmarshal(respBody, &resp)
	if resp.Error != nil {
		t.Fatalf("Unexpected error: %v", resp.Error)
	}

	resMap := resp.Result.(map[string]interface{})
	if resMap["content"] != "world" {
		t.Fatalf("Expected 'world', got %v", resMap["content"])
	}
}
