package tools

import (
	"context"
	"encoding/json"
	"testing"
)

type mockTool struct {
	name string
}

func (m *mockTool) Name() string {
	return m.name
}

func (m *mockTool) Description() string {
	return "Mock tool description"
}

func (m *mockTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{}`)
}

func (m *mockTool) Execute(ctx context.Context, args json.RawMessage) (string, error) {
	return "success", nil
}

func TestToolRegistry(t *testing.T) {
	registry := NewRegistry()

	// Test Register
	tool1 := &mockTool{name: "Tool1"}
	err := registry.Register(tool1)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Test Register Duplicate
	err = registry.Register(tool1)
	if err == nil {
		t.Fatalf("Expected error registering duplicate tool, got nil")
	}

	// Test Register Nil Tool
	err = registry.Register(nil)
	if err == nil {
		t.Fatalf("Expected error registering nil tool, got nil")
	}

	// Test Register Tool with empty name
	toolEmpty := &mockTool{name: ""}
	err = registry.Register(toolEmpty)
	if err == nil {
		t.Fatalf("Expected error registering tool with empty name, got nil")
	}

	// Test Get
	retrieved, exists := registry.Get("Tool1")
	if !exists {
		t.Fatalf("Expected tool to exist")
	}
	if retrieved.Name() != "Tool1" {
		t.Fatalf("Expected name Tool1, got %s", retrieved.Name())
	}

	// Test Get Non-existent
	_, exists = registry.Get("NonExistent")
	if exists {
		t.Fatalf("Expected tool to not exist")
	}

	// Test All
	tool2 := &mockTool{name: "Tool2"}
	registry.Register(tool2)

	allTools := registry.All()
	if len(allTools) != 2 {
		t.Fatalf("Expected 2 tools, got %d", len(allTools))
	}
}

func TestLegacyWrapper(t *testing.T) {
	wrapper := &LegacyWrapper{
		NameVal:        "LegacyTool",
		DescriptionVal: "A legacy tool",
		ParametersVal:  json.RawMessage(`{"type":"object"}`),
		ExecuteFn: func(ctx context.Context, args json.RawMessage) (string, error) {
			return "executed", nil
		},
	}

	if wrapper.Name() != "LegacyTool" {
		t.Errorf("Expected Name() to be LegacyTool")
	}
	if wrapper.Description() != "A legacy tool" {
		t.Errorf("Expected Description() to be 'A legacy tool'")
	}

	res, err := wrapper.Execute(context.Background(), nil)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	if res != "executed" {
		t.Errorf("Expected 'executed', got %s", res)
	}
}
