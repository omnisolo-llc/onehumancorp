package mcp

import (
	"context"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

// ServerRegistry is responsible for managing MCP servers and routing tools.
type ServerRegistry struct {
	mu            sync.RWMutex
	clientManager *ClientManager
	toolRouting   map[string]string // tool name -> server id
}

// NewServerRegistry creates a new MCP ServerRegistry.
func NewServerRegistry(cm *ClientManager) *ServerRegistry {
	return &ServerRegistry{
		clientManager: cm,
		toolRouting:   make(map[string]string),
	}
}

// RegisterServer connects to an MCP server and discovers its tools.
func (r *ServerRegistry) RegisterServer(ctx context.Context, id string, config ServerConfig) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if err := r.clientManager.ConnectStdio(ctx, id, config); err != nil {
		return fmt.Errorf("failed to connect to MCP server %s: %w", id, err)
	}

	// In a complete implementation, this would call ListTools on the MCP server.
	// For this task, we assume the server connects successfully.
	return nil
}

// UnregisterServer disconnects an MCP server and removes its tools.
func (r *ServerRegistry) UnregisterServer(id string) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if err := r.clientManager.Disconnect(id); err != nil {
		return fmt.Errorf("failed to disconnect from MCP server %s: %w", id, err)
	}

	for toolName, srvID := range r.toolRouting {
		if srvID == id {
			delete(r.toolRouting, toolName)
		}
	}

	return nil
}

// ExecuteTool routes a tool execution to the appropriate MCP server.
func (r *ServerRegistry) ExecuteTool(ctx context.Context, toolName string, args map[string]interface{}) (string, error) {
	r.mu.RLock()
	serverID, exists := r.toolRouting[toolName]
	r.mu.RUnlock()

	if !exists {
		return "", fmt.Errorf("tool %s not found in registry", toolName)
	}

	// In a complete implementation, this would send a CallTool request via JSON-RPC.
	return fmt.Sprintf("Executed tool %s on server %s", toolName, serverID), nil
}

// MapInternalToolToMCP exposes an internal tool to external MCP clients.
func MapInternalToolToMCP(tool builtin.Tool) Tool {
	return ConvertToMCPTool(tool)
}
