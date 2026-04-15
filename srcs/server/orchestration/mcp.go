package orchestration

import (
	"context"
	"fmt"
	"sync"
)

type MCPTool interface {
	Name() string
	Execute(ctx context.Context, input []byte) ([]byte, error)
}

var (
	mcpRegistry = make(map[string]MCPTool)
	mcpMu       sync.RWMutex
)

func RegisterMCPTool(tool MCPTool) error {
	mcpMu.Lock()
	defer mcpMu.Unlock()
	if _, exists := mcpRegistry[tool.Name()]; exists {
		return fmt.Errorf("MCP tool %s already registered", tool.Name())
	}
	mcpRegistry[tool.Name()] = tool
	return nil
}

func GetMCPTool(name string) (MCPTool, error) {
	mcpMu.RLock()
	defer mcpMu.RUnlock()
	if tool, exists := mcpRegistry[name]; exists {
		return tool, nil
	}
	return nil, fmt.Errorf("MCP tool %s not found", name)
}
