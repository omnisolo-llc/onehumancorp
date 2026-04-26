package mcp

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/agents/local"
)


type mockTool struct {
	name        string
	description string
	params      map[string]interface{}
}

func (m *mockTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{
		Name:        m.name,
		Description: m.description,
		InputSchema: m.params,
	}
}

func (m *mockTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	return "", nil
}

func TestConvertToMCPTool(t *testing.T) {
	internalTool := &mockTool{
		name:        "test_tool",
		description: "A test tool",
		params:      map[string]interface{}{"type": "object"},
	}

	mcpTool := ConvertToMCPTool(internalTool)

	if mcpTool.Name != "test_tool" {
		t.Errorf("Expected Name test_tool, got %s", mcpTool.Name)
	}
	if mcpTool.Description != "A test tool" {
		t.Errorf("Expected Description 'A test tool', got %s", mcpTool.Description)
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
