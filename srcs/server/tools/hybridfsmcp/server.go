package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

func (s *Server) HandleRequest(ctx context.Context, toolName string, params json.RawMessage) *mcp.ExecutionResult {
	switch toolName {
	case "read_file":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(params, &req); err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		data, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolName, "success", data, false)

	case "write_file":
		var req struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(params, &req); err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		if err := s.provider.WriteFile(ctx, req.Path, []byte(req.Content)); err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolName, "success", []byte("File written successfully"), false)

	case "list_directory":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(params, &req); err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		entries, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
		}
		res, _ := json.Marshal(entries)
		return mcp.FormatExecutionResult(toolName, "success", res, false)

	default:
		return mcp.FormatExecutionResult(toolName, "error", []byte(fmt.Sprintf("unknown tool: %s", toolName)), false)
	}
}
