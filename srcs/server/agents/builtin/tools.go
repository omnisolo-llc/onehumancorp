package builtin

import (
	"context"
	"encoding/json"
)

// Tool represents an executable function the agent can call.
type Tool struct {
	Name        string
	Description string
	SearchHint  string
	Parameters  json.RawMessage // JSON Schema of parameters
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
	RequiresApproval bool // Permission model flag
}

// ToolRegistry holds the available tools and manages permissions
type ToolRegistry struct {
	Tools map[string]Tool
}

// NewToolRegistry initializes the tool registry
func NewToolRegistry() *ToolRegistry {
	return &ToolRegistry{
		Tools: make(map[string]Tool),
	}
}

// Register adds a tool to the registry
func (tr *ToolRegistry) Register(t Tool) {
	tr.Tools[t.Name] = t
}

// Get returns a tool by name
func (tr *ToolRegistry) Get(name string) (Tool, bool) {
	t, ok := tr.Tools[name]
	return t, ok
}

// GetAll returns all tools
func (tr *ToolRegistry) GetAll() []Tool {
	var tools []Tool
	for _, t := range tr.Tools {
		tools = append(tools, t)
	}
	return tools
}
