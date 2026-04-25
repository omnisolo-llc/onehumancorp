package localstatefulproxy

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func TestProxyTool_Info(t *testing.T) {
	tool := NewProxyTool()
	info := tool.Info()

	if info.Name != "local_stateful_proxy" {
		t.Errorf("Expected Name 'local_stateful_proxy', got '%s'", info.Name)
	}

	if !strings.Contains(info.Description, "Proxies execution commands") {
		t.Errorf("Description does not match expected prefix, got: %s", info.Description)
	}

	schema, ok := info.InputSchema.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected InputSchema to be map[string]interface{}, got %T", info.InputSchema)
	}

	props, ok := schema["properties"].(map[string]interface{})
	if !ok {
		t.Fatalf("Expected InputSchema Properties to be map[string]interface{}, got %T", schema["properties"])
	}

	if _, ok := props["command"]; !ok {
		t.Errorf("Expected 'command' property in InputSchema")
	}

	if _, ok := props["context_id"]; !ok {
		t.Errorf("Expected 'context_id' property in InputSchema")
	}
}

func TestProxyTool_Execute_Success(t *testing.T) {
	// Use the underlying ProxyTool instead of an AgentProxyTool
	// since we want to test ProxyTool itself here directly.
	tool := NewProxyTool()
	ctx := context.Background()

	args := map[string]interface{}{
		"command":    "SELECT * FROM users",
		"context_id": "tenant-123-shard-1",
	}

	result, err := tool.Execute(ctx, args)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if result == nil {
		t.Fatal("Expected result, got nil")
	}

	if len(result.Content) != 1 {
		t.Fatalf("Expected 1 content block, got %d", len(result.Content))
	}

	textContent, ok := result.Content[0].(*mcp.TextContent)
	if !ok {
		t.Fatalf("Expected *mcp.TextContent, got %T", result.Content[0])
	}

	var responseData map[string]string
	if err := json.Unmarshal([]byte(textContent.Text), &responseData); err != nil {
		t.Fatalf("Failed to unmarshal response text: %v", err)
	}

	if responseData["status"] != "success" {
		t.Errorf("Expected status 'success', got '%s'", responseData["status"])
	}

	expectedMessage := "Command 'SELECT * FROM users' successfully proxied to context 'tenant-123-shard-1'"
	if responseData["message"] != expectedMessage {
		t.Errorf("Expected message '%s', got '%s'", expectedMessage, responseData["message"])
	}
}

func TestProxyTool_Execute_MissingCommand(t *testing.T) {
	tool := NewProxyTool()
	ctx := context.Background()

	args := map[string]interface{}{
		"context_id": "tenant-123-shard-1",
	}

	_, err := tool.Execute(ctx, args)
	if err == nil {
		t.Fatal("Expected error for missing command, got nil")
	}

	if !strings.Contains(err.Error(), "missing or invalid 'command'") {
		t.Errorf("Expected error message to contain 'missing or invalid \\'command\\'', got '%v'", err)
	}
}

func TestProxyTool_Execute_InvalidCommandType(t *testing.T) {
	tool := NewProxyTool()
	ctx := context.Background()

	args := map[string]interface{}{
		"command":    123, // Invalid type
		"context_id": "tenant-123-shard-1",
	}

	_, err := tool.Execute(ctx, args)
	if err == nil {
		t.Fatal("Expected error for invalid command type, got nil")
	}
}

func TestProxyTool_Execute_MissingContextID(t *testing.T) {
	tool := NewProxyTool()
	ctx := context.Background()

	args := map[string]interface{}{
		"command": "SELECT * FROM users",
	}

	_, err := tool.Execute(ctx, args)
	if err == nil {
		t.Fatal("Expected error for missing context_id, got nil")
	}

	if !strings.Contains(err.Error(), "missing or invalid 'context_id'") {
		t.Errorf("Expected error message to contain 'missing or invalid \\'context_id\\'', got '%v'", err)
	}
}
