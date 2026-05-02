package tools

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
	"time"
)

// mockTool is a simple mock tool for testing purposes.
type mockTool struct {
	name        string
	description string
	schema      json.RawMessage
	executeFn   func(ctx context.Context, input []byte) ([]byte, error)
}

func (m *mockTool) Name() string {
	return m.name
}

func (m *mockTool) Description() string {
	return m.description
}

func (m *mockTool) InputSchema() json.RawMessage {
	return m.schema
}

func (m *mockTool) Execute(ctx context.Context, input []byte) ([]byte, error) {
	if m.executeFn != nil {
		return m.executeFn(ctx, input)
	}
	return []byte("mock output"), nil
}

func TestBaseRegistry(t *testing.T) {
	registry := NewRegistry()

	t.Run("Register Tool", func(t *testing.T) {
		tool1 := &mockTool{
			name:        "test-tool-1",
			description: "A test tool",
			schema:      json.RawMessage(`{"type": "object"}`),
		}

		err := registry.Register(tool1)
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		// Try registering again, should fail
		err = registry.Register(tool1)
		if err == nil {
			t.Fatal("expected error when registering duplicate tool")
		}
		if !strings.Contains(err.Error(), "already registered") {
			t.Fatalf("expected already registered error, got: %v", err)
		}
	})

	t.Run("Get Tool", func(t *testing.T) {
		tool, exists := registry.GetTool("test-tool-1")
		if !exists {
			t.Fatal("expected tool to exist")
		}
		if tool.Name() != "test-tool-1" {
			t.Fatalf("expected tool name 'test-tool-1', got: %s", tool.Name())
		}

		_, exists = registry.GetTool("non-existent")
		if exists {
			t.Fatal("expected non-existent tool to not exist")
		}
	})

	t.Run("List Tools", func(t *testing.T) {
		tool2 := &mockTool{
			name:        "test-tool-2",
			description: "Another test tool",
			schema:      json.RawMessage(`{"type": "object"}`),
		}
		_ = registry.Register(tool2)

		tools := registry.ListTools()
		if len(tools) != 2 {
			t.Fatalf("expected 2 tools, got: %d", len(tools))
		}

		names := make(map[string]bool)
		for _, t := range tools {
			names[t.Name()] = true
		}
		if !names["test-tool-1"] || !names["test-tool-2"] {
			t.Fatal("expected test-tool-1 and test-tool-2 in list")
		}
	})

	t.Run("Execute Tool", func(t *testing.T) {
		ctx := context.Background()

		// Execute existing tool
		output, err := registry.ExecuteTool(ctx, "test-tool-1", []byte("input"))
		if err != nil {
			t.Fatalf("expected no error executing tool, got: %v", err)
		}
		if string(output) != "mock output" {
			t.Fatalf("expected 'mock output', got: %s", string(output))
		}

		// Execute with timeout check
		tool3 := &mockTool{
			name: "timeout-tool",
			executeFn: func(ctx context.Context, input []byte) ([]byte, error) {
				select {
				case <-ctx.Done():
					return nil, ctx.Err()
				case <-time.After(10 * time.Millisecond):
					return []byte("done"), nil
				}
			},
		}
		_ = registry.Register(tool3)

		ctxTimeout, cancel := context.WithTimeout(context.Background(), 1*time.Millisecond)
		defer cancel()

		_, err = registry.ExecuteTool(ctxTimeout, "timeout-tool", nil)
		if err == nil {
			t.Fatal("expected context deadline exceeded error")
		}

		// Execute non-existent tool
		_, err = registry.ExecuteTool(ctx, "non-existent", nil)
		if err == nil {
			t.Fatal("expected error executing non-existent tool")
		}
	})
}
