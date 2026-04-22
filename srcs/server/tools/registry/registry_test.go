package registry

import (
	"context"
	"encoding/json"
	"testing"
)

type mockTool struct {
	name        string
	description string
	schema      json.RawMessage
	executeFunc func(ctx context.Context, input json.RawMessage) (json.RawMessage, error)
}

func (m *mockTool) Name() string { return m.name }
func (m *mockTool) Description() string { return m.description }
func (m *mockTool) InputSchema() json.RawMessage { return m.schema }
func (m *mockTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	if m.executeFunc != nil {
		return m.executeFunc(ctx, input)
	}
	return nil, nil
}

func TestUnifiedToolRegistry_RegisterTool(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	validSchema := json.RawMessage(`{"type": "object", "properties": {"msg": {"type": "string"}}}`)
	tool := &mockTool{
		name:        "test_tool",
		description: "A test tool",
		schema:      validSchema,
	}

	err := registry.RegisterTool(tool)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Try registering again
	err = registry.RegisterTool(tool)
	if err == nil {
		t.Fatal("expected error when registering duplicate tool, got nil")
	}

	// Try registering with invalid schema
	invalidTool := &mockTool{
		name:        "invalid_tool",
		description: "An invalid tool",
		schema:      json.RawMessage(`{"type": "invalid_type"}`),
	}
	err = registry.RegisterTool(invalidTool)
	if err == nil {
		t.Fatal("expected error when registering tool with invalid schema, got nil")
	}
}

func TestUnifiedToolRegistry_ListTools(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	tool1 := &mockTool{name: "tool1", schema: json.RawMessage(`{}`)}
	tool2 := &mockTool{name: "tool2", schema: json.RawMessage(`{}`)}

	registry.RegisterTool(tool1)
	registry.RegisterTool(tool2)

	manifests := registry.ListTools()
	if len(manifests) != 2 {
		t.Fatalf("expected 2 tools, got %d", len(manifests))
	}
}

func TestUnifiedToolRegistry_Execute(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	expectedOutput := json.RawMessage(`{"status": "ok"}`)
	tool := &mockTool{
		name:   "execute_tool",
		schema: json.RawMessage(`{}`),
		executeFunc: func(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
			return expectedOutput, nil
		},
	}

	registry.RegisterTool(tool)

	output, err := registry.Execute(context.Background(), "execute_tool", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if string(output) != string(expectedOutput) {
		t.Fatalf("expected %s, got %s", expectedOutput, output)
	}

	// Execute missing tool
	_, err = registry.Execute(context.Background(), "missing_tool", nil)
	if err == nil {
		t.Fatal("expected error when executing missing tool, got nil")
	}
}

func TestUnifiedToolRegistry_RegisterTool_CompileError(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	// An invalid JSON schema according to Draft 2020-12
	invalidSchema := json.RawMessage(`{"type": "object", "required": "not_an_array"}`)
	tool := &mockTool{
		name:        "invalid_tool",
		description: "An invalid tool",
		schema:      invalidSchema,
	}

	err := registry.RegisterTool(tool)
	if err == nil {
		t.Fatal("expected error when compiling invalid schema, got nil")
	}
}

func TestUnifiedToolRegistry_RegisterTool_AddResourceError(t *testing.T) {
	registry := NewUnifiedToolRegistry()

	// An invalid JSON that will fail AddResource
	invalidJSON := json.RawMessage(`{not_json`)
	tool := &mockTool{
		name:        "invalid_json_tool",
		description: "A tool with invalid JSON schema",
		schema:      invalidJSON,
	}

	err := registry.RegisterTool(tool)
	if err == nil {
		t.Fatal("expected error when adding invalid json resource, got nil")
	}
}
