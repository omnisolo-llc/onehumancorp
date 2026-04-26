package mcp

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/agents/local"
)

type testTool struct{}

func (t *testTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{
		Name:        "test_tool",
		Description: "A test tool",
		InputSchema: map[string]interface{}{"type":"object","properties":map[string]interface{}{}},
	}
}

func (t *testTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	return "", nil
}

func TestConvertToMCPTool(t *testing.T) {
	internalTool := &testTool{}

	mcpTool := ConvertToMCPTool(internalTool)

	if mcpTool.Name != "test_tool" {
		t.Errorf("Expected Name test_tool, got %s", mcpTool.Name)
	}
	if mcpTool.Description != "A test tool" {
		t.Errorf("Expected Description 'A test tool', got %s", mcpTool.Description)
	}

	// Workaround to test both stringified json combinations because object properties order is undetermined
	if string(mcpTool.InputSchema) != `{"type":"object","properties":{}}` && string(mcpTool.InputSchema) != `{"properties":{},"type":"object"}` {

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
