package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os/exec"
	"sync"

	"onehumancorp/srcs/server/telemetry"
)

// InternalTool is the internal representation of an OHC tool.
type InternalTool struct {
	Name        string                 `json:"name"`
	Description string                 `json:"description"`
	Parameters  map[string]interface{} `json:"parameters"`
}

// Tool is the MCP specification for a tool.
type Tool struct {
	Name        string                 `json:"name"`
	Description string                 `json:"description"`
	InputSchema map[string]interface{} `json:"inputSchema"`
}

// ConvertToMCPTool maps our internal tools to the MCP tool specification.
func ConvertToMCPTool(t InternalTool) Tool {
	return Tool{
		Name:        t.Name,
		Description: t.Description,
		InputSchema: t.Parameters,
	}
}

// ServerConfig configures an MCP server connection.
type ServerConfig struct {
	ID      string
	Command string
	Args    []string
	Env     []string
}

// JSONRPCMessage represents a standard JSON-RPC 2.0 message used by MCP
type JSONRPCMessage struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      *string         `json:"id,omitempty"`
	Method  string          `json:"method,omitempty"`
	Params  json.RawMessage `json:"params,omitempty"`
	Result  json.RawMessage `json:"result,omitempty"`
	Error   *JSONRPCError   `json:"error,omitempty"`
}

// JSONRPCError represents a JSON-RPC error.
type JSONRPCError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// CallToolRequestParams represents the parameters for a call tool request
type CallToolRequestParams struct {
	Name      string                 `json:"name"`
	Arguments map[string]interface{} `json:"arguments"`
}

// CallToolResult represents the result from a tool call
type CallToolResult struct {
	Content []interface{} `json:"content"`
	IsError bool          `json:"isError,omitempty"`
}

// ListToolsResult represents the result of listing tools
type ListToolsResult struct {
	Tools []Tool `json:"tools"`
}

// ClientManager manages connections to MCP servers.
type ClientManager struct {
	servers map[string]*MCPServer
	mu      sync.RWMutex
}

// MCPServer represents a connected MCP server.
type MCPServer struct {
	Config  ServerConfig
	cmd      *exec.Cmd
	stdin    io.WriteCloser
	stdout   io.ReadCloser
	decoder  *json.Decoder
	mu       sync.Mutex
	reqID    int
	requests map[string]chan *JSONRPCMessage
	ctx      context.Context
	cancel   context.CancelFunc
}

// NewClientManager creates a new MCP client manager.
func NewClientManager() *ClientManager {
	return &ClientManager{
		servers: make(map[string]*MCPServer),
	}
}

// ConnectStdio spawns an MCP server using stdio.
func (cm *ClientManager) ConnectStdio(ctx context.Context, config ServerConfig) error {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	if _, exists := cm.servers[config.ID]; exists {
		return fmt.Errorf("server %s already connected", config.ID)
	}

	cmd := exec.CommandContext(ctx, config.Command, config.Args...)
	cmd.Env = config.Env

	stdin, err := cmd.StdinPipe()
	if err != nil {
		return fmt.Errorf("failed to get stdin pipe: %w", err)
	}

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return fmt.Errorf("failed to get stdout pipe: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start server: %w", err)
	}

	serverCtx, cancel := context.WithCancel(ctx)

	server := &MCPServer{
		Config:   config,
		cmd:      cmd,
		stdin:    stdin,
		stdout:   stdout,
		decoder:  json.NewDecoder(stdout),
		requests: make(map[string]chan *JSONRPCMessage),
		ctx:      serverCtx,
		cancel:   cancel,
	}

	go server.readLoop()

	go func() {
		// Wait for command to finish to avoid zombie processes
		cmd.Wait()
		server.cancel()
		cm.Disconnect(config.ID)
	}()

	cm.servers[config.ID] = server

	return nil
}

// Disconnect cleans up a server connection.
func (cm *ClientManager) Disconnect(serverID string) {
	cm.mu.Lock()
	server, ok := cm.servers[serverID]
	if ok {
		delete(cm.servers, serverID)
	}
	cm.mu.Unlock()

	if ok {
		server.cancel()
		if server.cmd != nil && server.cmd.Process != nil {
			server.cmd.Process.Kill()
		}
	}
}

// CallTool calls a tool on a specific MCP server.
func (cm *ClientManager) CallTool(ctx context.Context, serverID string, name string, args map[string]interface{}) (*CallToolResult, error) {
	cm.mu.RLock()
	server, ok := cm.servers[serverID]
	cm.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("server %s not found", serverID)
	}

	telemetry.RecordMCPToolCall(ctx, name)

	reqIDStr := fmt.Sprintf("%d", server.nextID())

	paramsBytes, err := json.Marshal(CallToolRequestParams{
		Name:      name,
		Arguments: args,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to marshal params: %w", err)
	}

	req := JSONRPCMessage{
		JSONRPC: "2.0",
		ID:      &reqIDStr,
		Method:  "tools/call",
		Params:  json.RawMessage(paramsBytes),
	}

	resp, err := server.sendRequest(ctx, req)
	if err != nil {
		return nil, err
	}

	if resp.Error != nil {
		return nil, fmt.Errorf("tool call error: %s (code: %d)", resp.Error.Message, resp.Error.Code)
	}

	var result CallToolResult
	if err := json.Unmarshal(resp.Result, &result); err != nil {
		return nil, fmt.Errorf("failed to unmarshal result: %w", err)
	}

	return &result, nil
}

// ListTools lists all tools available on a specific MCP server.
func (cm *ClientManager) ListTools(ctx context.Context, serverID string) ([]Tool, error) {
	cm.mu.RLock()
	server, ok := cm.servers[serverID]
	cm.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("server %s not found", serverID)
	}

	reqIDStr := fmt.Sprintf("%d", server.nextID())
	req := JSONRPCMessage{
		JSONRPC: "2.0",
		ID:      &reqIDStr,
		Method:  "tools/list",
	}

	resp, err := server.sendRequest(ctx, req)
	if err != nil {
		return nil, err
	}

	if resp.Error != nil {
		return nil, fmt.Errorf("list tools error: %s (code: %d)", resp.Error.Message, resp.Error.Code)
	}

	var result ListToolsResult
	if err := json.Unmarshal(resp.Result, &result); err != nil {
		return nil, fmt.Errorf("failed to unmarshal result: %w", err)
	}

	return result.Tools, nil
}

func (s *MCPServer) nextID() int {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.reqID++
	return s.reqID
}

func (s *MCPServer) readLoop() {
	defer s.cancel()
	for {
		var msg JSONRPCMessage
		if err := s.decoder.Decode(&msg); err != nil {
			if err == io.EOF {
				return
			}
			// In a real application, we might want to log this error
			return
		}

		if msg.ID != nil {
			s.mu.Lock()
			ch, ok := s.requests[*msg.ID]
			if ok {
				delete(s.requests, *msg.ID)
			}
			s.mu.Unlock()

			if ok {
				ch <- &msg
			}
		}
	}
}

func (s *MCPServer) sendRequest(ctx context.Context, req JSONRPCMessage) (*JSONRPCMessage, error) {
	if req.ID == nil {
		return nil, fmt.Errorf("request ID is required")
	}

	ch := make(chan *JSONRPCMessage, 1)

	s.mu.Lock()
	s.requests[*req.ID] = ch
	s.mu.Unlock()

	reqBytes, err := json.Marshal(req)
	if err != nil {
		s.mu.Lock()
		delete(s.requests, *req.ID)
		s.mu.Unlock()
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	// Add newline to flush json over stdio
	reqBytes = append(reqBytes, '\n')

	// Write needs to be serialized to prevent garbled requests
	s.mu.Lock()
	_, writeErr := s.stdin.Write(reqBytes)
	s.mu.Unlock()

	if writeErr != nil {
		s.mu.Lock()
		delete(s.requests, *req.ID)
		s.mu.Unlock()
		return nil, fmt.Errorf("failed to write request: %w", writeErr)
	}

	select {
	case resp := <-ch:
		return resp, nil
	case <-ctx.Done():
		s.mu.Lock()
		delete(s.requests, *req.ID)
		s.mu.Unlock()
		return nil, ctx.Err()
	case <-s.ctx.Done():
		s.mu.Lock()
		delete(s.requests, *req.ID)
		s.mu.Unlock()
		return nil, fmt.Errorf("server context cancelled or closed")
	}
}
