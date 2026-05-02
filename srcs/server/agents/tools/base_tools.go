package tools

import (
	"context"
	"encoding/json"
)

// CalculatorTool is a simple base tool.
type CalculatorTool struct{}

func (c *CalculatorTool) Name() string {
	return "local-calculator"
}

func (c *CalculatorTool) Description() string {
	return "A local calculator tool"
}

func (c *CalculatorTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{"type": "object", "properties": {"expression": {"type": "string"}}}`)
}

func (c *CalculatorTool) Execute(ctx context.Context, input []byte) ([]byte, error) {
	return []byte("calculated result"), nil
}

// GrepTool is a simple base tool for searching.
type GrepTool struct{}

func (g *GrepTool) Name() string {
	return "local-grep"
}

func (g *GrepTool) Description() string {
	return "Local file search tool"
}

func (g *GrepTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{"type": "object", "properties": {"pattern": {"type": "string"}, "path": {"type": "string"}}}`)
}

func (g *GrepTool) Execute(ctx context.Context, input []byte) ([]byte, error) {
	return []byte("grep result"), nil
}

// NewDefaultRegistry creates a registry populated with the base tools.
func NewDefaultRegistry() *BaseRegistry {
	registry := NewRegistry()
	_ = registry.Register(&CalculatorTool{})
	_ = registry.Register(&GrepTool{})
	return registry
}
