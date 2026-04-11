package mcp

import (
	"sync"
)

// DefaultRegistry is a global registry of MCP tools.
var DefaultRegistry = &Registry{
	tools: make(map[string]Tool),
}

// Registry manages MCP tools.
type Registry struct {
	mu    sync.RWMutex
	tools map[string]Tool
}

// Register registers a tool.
func (r *Registry) Register(tool Tool) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.tools[tool.Name] = tool
}

// Get gets a tool.
func (r *Registry) Get(name string) (Tool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()
	tool, ok := r.tools[name]
	return tool, ok
}
