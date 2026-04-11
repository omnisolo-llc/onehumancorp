package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// Server exposes filesystem operations as MCP tools
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new Hybrid FS MCP Server
func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// Execute handles tool executions for the Hybrid FS MCP server
func (s *Server) Execute(ctx context.Context, toolID string, params json.RawMessage) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		return s.handleReadFile(ctx, params)
	case "write_file":
		return s.handleWriteFile(ctx, params)
	case "list_directory":
		return s.handleListDirectory(ctx, params)
	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("unknown tool_id: %s", toolID)), false)
	}
}

func (s *Server) handleReadFile(ctx context.Context, params json.RawMessage) *mcp.ExecutionResult {
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(params, &args); err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("invalid params: %v", err)), false)
	}

	data, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("failed to read file: %v", err)), false)
	}

	result := struct {
		Content string `json:"content"`
	}{
		Content: string(data),
	}

	resultBytes, _ := json.Marshal(result)
	return mcp.FormatExecutionResult("read_file", "success", resultBytes, false)
}

func (s *Server) handleWriteFile(ctx context.Context, params json.RawMessage) *mcp.ExecutionResult {
	var args struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(params, &args); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("invalid params: %v", err)), false)
	}

	if err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content)); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("failed to write file: %v", err)), false)
	}

	result := struct {
		Success bool `json:"success"`
	}{
		Success: true,
	}

	resultBytes, _ := json.Marshal(result)
	return mcp.FormatExecutionResult("write_file", "success", resultBytes, false)
}

func (s *Server) handleListDirectory(ctx context.Context, params json.RawMessage) *mcp.ExecutionResult {
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(params, &args); err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("invalid params: %v", err)), false)
	}

	entries, err := s.provider.ListDir(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("failed to list directory: %v", err)), false)
	}

	var files []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		files = append(files, name)
	}

	result := struct {
		Files []string `json:"files"`
	}{
		Files: files,
	}

	resultBytes, _ := json.Marshal(result)
	return mcp.FormatExecutionResult("list_directory", "success", resultBytes, false)
}
