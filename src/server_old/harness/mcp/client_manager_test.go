package mcp

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/agents/local"
)

func TestConvertToMCPTool(t *testing.T) {
	internalTool := local.ToolDefinition{
		Name:        "test_tool",
		Description: "A test tool",
		InputSchema: map[string]interface{}{
			"type":       "object",
			"properties": map[string]interface{}{},
		},
	}

	mcpTool := ConvertToMCPTool(internalTool)

	if mcpTool.Name != "test_tool" {
		t.Errorf("Expected Name test_tool, got %s", mcpTool.Name)
	}
	if mcpTool.Description != "A test tool" {
		t.Errorf("Expected Description 'A test tool', got %s", mcpTool.Description)
	}
	if string(mcpTool.InputSchema) != `{"type":"object","properties":{}}` {
		t.Errorf("Expected InputSchema `{\"type\":\"object\",\"properties\":{}}`, got %s", string(mcpTool.InputSchema))
	}
}

func TestClientManagerConnectStdio(t *testing.T) {
	cm := NewClientManager()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	config := ServerConfig{
		Command: "echo",
		Args:    []string{"hello"},
	}

	err := cm.ConnectStdio(ctx, "test_server", config)
	if err != nil {
		t.Fatalf("Failed to connect stdio: %v", err)
	}

	err = cm.Disconnect("test_server")
	if err != nil {
		t.Fatalf("Failed to disconnect: %v", err)
	}
}
