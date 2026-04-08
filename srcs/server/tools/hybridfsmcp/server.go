package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// MCPRegistry defines a simple registry interface to mimic an MCP registry
// Since mcp.Registry doesn't actually exist in the codebase, we'll create an interface
// or a simple struct to handle tool registration and execution.
type Registry struct {
	mu    sync.RWMutex
	tools map[string]ToolHandler
}

type ToolHandler func(ctx context.Context, args json.RawMessage) *mcp.ExecutionResult

func NewRegistry() *Registry {
	return &Registry{
		tools: make(map[string]ToolHandler),
	}
}

func (r *Registry) RegisterTool(name, description string, handler ToolHandler) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.tools[name] = handler
}

func (r *Registry) ExecuteTool(ctx context.Context, name string, args json.RawMessage) (*mcp.ExecutionResult, error) {
	r.mu.RLock()
	handler, ok := r.tools[name]
	r.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("tool not found: %s", name)
	}

	return handler(ctx, args), nil
}


// Server wraps a FileSystemProvider to expose MCP tools
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new Hybrid FS MCP Server
func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

// RegisterTools registers the file system tools with the MCP registry
func (s *Server) RegisterTools(registry *Registry) {
	registry.RegisterTool("read_file", "Read a file from the hybrid file system", s.handleReadFile)
	registry.RegisterTool("write_file", "Write data to a file in the hybrid file system", s.handleWriteFile)
	registry.RegisterTool("list_directory", "List contents of a directory in the hybrid file system", s.handleListDir)
}

func (s *Server) handleReadFile(ctx context.Context, args json.RawMessage) *mcp.ExecutionResult {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	data, err := s.provider.ReadFile(ctx, input.Path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("failed to read file: %v", err)), false)
	}

	return mcp.FormatExecutionResult("read_file", "success", data, false)
}

func (s *Server) handleWriteFile(ctx context.Context, args json.RawMessage) *mcp.ExecutionResult {
	var input struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	err := s.provider.WriteFile(ctx, input.Path, []byte(input.Content))
	if err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("failed to write file: %v", err)), false)
	}

	return mcp.FormatExecutionResult("write_file", "success", []byte("file written successfully"), false)
}

func (s *Server) handleListDir(ctx context.Context, args json.RawMessage) *mcp.ExecutionResult {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	entries, err := s.provider.ListDir(ctx, input.Path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("failed to list directory: %v", err)), false)
	}

	resultData, err := json.Marshal(entries)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("failed to marshal result: %v", err)), false)
	}

	return mcp.FormatExecutionResult("list_directory", "success", resultData, false)
}
