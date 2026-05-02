package registry

import (
	"context"
	"encoding/json"
	"testing"
)

type MockTool struct {
	name        string
	description string
	schema      json.RawMessage
}

func (m *MockTool) Name() string {
	return m.name
}

func (m *MockTool) Description() string {
	return m.description
}

func (m *MockTool) InputSchema() json.RawMessage {
	return m.schema
}

func (m *MockTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	return nil, nil
}

func TestUnifiedToolRegistry_RegisterTool(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	tests := []struct {
		name    string
		tool    AgentTool
		wantErr bool
	}{
		{
			name: "Valid Tool",
			tool: &MockTool{
				name:        "test_tool",
				description: "A test tool",
				schema:      []byte(`{"type":"object"}`),
			},
			wantErr: false,
		},
		{
			name: "Empty Name",
			tool: &MockTool{
				name:        "",
				description: "A test tool",
				schema:      []byte(`{"type":"object"}`),
			},
			wantErr: true,
		},
		{
			name: "Empty Schema",
			tool: &MockTool{
				name:        "test_tool_empty_schema",
				description: "A test tool",
				schema:      []byte(``),
			},
			wantErr: true,
		},
		{
			name: "Invalid JSON Schema",
			tool: &MockTool{
				name:        "test_tool_invalid_schema",
				description: "A test tool",
				schema:      []byte(`{"type":"object"`), // missing closing brace
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := registry.RegisterTool(tt.tool)
			if (err != nil) != tt.wantErr {
				t.Errorf("RegisterTool() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestUnifiedToolRegistry_ListTools(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	tool1 := &MockTool{
		name:        "tool1",
		description: "desc1",
		schema:      []byte(`{"type":"object"}`),
	}
	tool2 := &MockTool{
		name:        "tool2",
		description: "desc2",
		schema:      []byte(`{"type":"string"}`),
	}

	_ = registry.RegisterTool(tool1)
	_ = registry.RegisterTool(tool2)

	tools := registry.ListTools()
	if len(tools) != 2 {
		t.Errorf("ListTools() returned %d tools, want 2", len(tools))
	}

	foundTool1 := false
	foundTool2 := false
	for _, manifest := range tools {
		if manifest.Name == "tool1" && manifest.Description == "desc1" {
			foundTool1 = true
		}
		if manifest.Name == "tool2" && manifest.Description == "desc2" {
			foundTool2 = true
		}
	}

	if !foundTool1 || !foundTool2 {
		t.Errorf("ListTools() did not return expected tools: %v", tools)
	}
}

func TestUnifiedToolRegistry_GetTool(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	tool := &MockTool{
		name:        "my_tool",
		description: "desc",
		schema:      []byte(`{"type":"object"}`),
	}

	_ = registry.RegisterTool(tool)

	got, ok := registry.GetTool("my_tool")
	if !ok {
		t.Errorf("GetTool() returned false for existing tool")
	}
	if got.Name() != "my_tool" {
		t.Errorf("GetTool() returned wrong tool name = %s, want my_tool", got.Name())
	}

	_, ok = registry.GetTool("nonexistent")
	if ok {
		t.Errorf("GetTool() returned true for nonexistent tool")
	}
}

func TestUnifiedToolRegistry_ExecuteTool(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	tool := &MockTool{
		name:        "my_tool",
		description: "desc",
		schema:      []byte(`{"type":"object"}`),
	}
	_ = registry.RegisterTool(tool)

	_, err := registry.ExecuteTool(context.Background(), "my_tool", []byte(`{}`))
	if err != nil {
		t.Errorf("ExecuteTool() returned error: %v", err)
	}

	_, err = registry.ExecuteTool(context.Background(), "nonexistent", []byte(`{}`))
	if err == nil {
		t.Errorf("ExecuteTool() expected error for nonexistent tool")
	}
}
