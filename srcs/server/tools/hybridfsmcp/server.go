package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	Provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{Provider: provider}
}

func (s *Server) HandleReadFile(ctx context.Context, path string) *mcp.ExecutionResult {
	data, err := s.Provider.ReadFile(ctx, path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
	}

	resData, _ := json.Marshal(map[string]interface{}{
		"content": string(data),
	})

	return mcp.FormatExecutionResult("read_file", "success", resData, false)
}

func (s *Server) HandleWriteFile(ctx context.Context, path string, data []byte) *mcp.ExecutionResult {
	err := s.Provider.WriteFile(ctx, path, data)
	if err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
	}

	resData, _ := json.Marshal(map[string]interface{}{
		"success": true,
	})

	return mcp.FormatExecutionResult("write_file", "success", resData, false)
}

func (s *Server) HandleListDirectory(ctx context.Context, path string) *mcp.ExecutionResult {
	files, err := s.Provider.ListDir(ctx, path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
	}

	resData, _ := json.Marshal(map[string]interface{}{
		"files": files,
	})

	return mcp.FormatExecutionResult("list_directory", "success", resData, false)
}
