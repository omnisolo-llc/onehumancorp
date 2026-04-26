package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os/exec"
	"sync"
)

// InternalTool represents the internal tool definition
type InternalTool struct {
	Name        string
	Description string
	Parameters  json.RawMessage
}

// Tool represents a Model Context Protocol tool spec.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ServerConfig defines the configuration for an external MCP server.
type ServerConfig struct {
	Command string   `json:"command"`
	Args    []string `json:"args"`
	Env     []string `json:"env"`
}

// ConvertToMCPTool maps an internal tool to the MCP Tool specification.
func ConvertToMCPTool(t InternalTool) Tool {
	return Tool{
		Name:        t.Name,
		Description: t.Description,
		InputSchema: t.Parameters,
	}
}

// ClientManager manages connections to MCP servers.
type ClientManager struct {
	mu      sync.Mutex
	servers map[string]*MCPServer
}

// NewClientManager creates a new MCP client manager.
func NewClientManager() *ClientManager {
	return &ClientManager{
		servers: make(map[string]*MCPServer),
	}
}

// MCPServer represents a running external MCP server.
type MCPServer struct {
	config ServerConfig
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout io.ReadCloser
}

// ConnectStdio spawns an MCP server using stdio transport.
func (cm *ClientManager) ConnectStdio(ctx context.Context, id string, config ServerConfig) error {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	if _, exists := cm.servers[id]; exists {
		return fmt.Errorf("server %s already connected", id)
	}

	cmd := exec.CommandContext(ctx, config.Command, config.Args...)
	cmd.Env = config.Env

	stdin, err := cmd.StdinPipe()
	if err != nil {
		return err
	}

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return err
	}

	if err := cmd.Start(); err != nil {
		return err
	}

	cm.servers[id] = &MCPServer{
		config: config,
		cmd:    cmd,
		stdin:  stdin,
		stdout: stdout,
	}

	return nil
}

// Disconnect stops an MCP server.
func (cm *ClientManager) Disconnect(id string) error {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	srv, exists := cm.servers[id]
	if !exists {
		return fmt.Errorf("server %s not found", id)
	}

	srv.stdin.Close()
	srv.cmd.Process.Kill()
	srv.cmd.Wait()

	delete(cm.servers, id)
	return nil
}
