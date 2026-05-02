package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
)

// Tool defines the interface for an executable agent tool.
type Tool interface {
	Name() string
	Description() string
	InputSchema() json.RawMessage
	Execute(ctx context.Context, input []byte) ([]byte, error)
}

// ToolRegistry defines the interface for a unified registry of tools.
type ToolRegistry interface {
	Register(tool Tool) error
	GetTool(name string) (Tool, bool)
	ListTools() []Tool
	ExecuteTool(ctx context.Context, name string, input []byte) ([]byte, error)
}

// BaseRegistry provides a concurrent-safe implementation of ToolRegistry.
type BaseRegistry struct {
	mu    sync.RWMutex
	tools map[string]Tool
}

// NewRegistry creates a new instance of BaseRegistry.
func NewRegistry() *BaseRegistry {
	return &BaseRegistry{
		tools: make(map[string]Tool),
	}
}

// Register adds a tool to the registry. Returns an error if a tool with the same name already exists.
func (r *BaseRegistry) Register(tool Tool) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	name := tool.Name()
	if _, exists := r.tools[name]; exists {
		return fmt.Errorf("tool '%s' is already registered", name)
	}

	r.tools[name] = tool
	return nil
}

// GetTool retrieves a tool by name.
func (r *BaseRegistry) GetTool(name string) (Tool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	tool, exists := r.tools[name]
	return tool, exists
}

// ListTools returns a list of all registered tools.
func (r *BaseRegistry) ListTools() []Tool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var toolList []Tool
	for _, tool := range r.tools {
		toolList = append(toolList, tool)
	}
	return toolList
}

// ExecuteTool executes a registered tool by name with the given input.
func (r *BaseRegistry) ExecuteTool(ctx context.Context, name string, input []byte) ([]byte, error) {
	tool, exists := r.GetTool(name)
	if !exists {
		return nil, fmt.Errorf("tool '%s' not found", name)
	}

	return tool.Execute(ctx, input)
}
