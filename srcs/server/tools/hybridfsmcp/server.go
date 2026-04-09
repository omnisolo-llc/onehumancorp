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

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"` // text data to be written
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *Server) HandleToolCall(ctx context.Context, toolName string, argsRaw []byte) *mcp.ExecutionResult {
	switch toolName {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.formatError(toolName, err)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return s.formatError(toolName, err)
		}
		return s.formatSuccess(toolName, map[string]string{"content": string(data)})

	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.formatError(toolName, err)
		}
		if err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data)); err != nil {
			return s.formatError(toolName, err)
		}
		return s.formatSuccess(toolName, map[string]string{"status": "success"})

	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.formatError(toolName, err)
		}
		entries, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return s.formatError(toolName, err)
		}
		return s.formatSuccess(toolName, map[string]interface{}{"entries": entries})

	default:
		return s.formatError(toolName, fmt.Errorf("unknown tool %s", toolName))
	}
}

func (s *Server) formatSuccess(toolID string, result interface{}) *mcp.ExecutionResult {
	data, _ := json.Marshal(result)
	return mcp.FormatExecutionResult(toolID, "success", data, false)
}

func (s *Server) formatError(toolID string, err error) *mcp.ExecutionResult {
	data, _ := json.Marshal(map[string]string{"error": err.Error()})
	return mcp.FormatExecutionResult(toolID, "error", data, false)
}
