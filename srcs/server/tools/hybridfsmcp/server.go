package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	return &FileSystemMCPServer{provider: provider}
}

func (s *FileSystemMCPServer) HandleReadFile(ctx context.Context, args json.RawMessage) (*mcp.ExecutionResult, error) {
	var params struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}

	data, err := s.provider.ReadFile(ctx, params.Path)
	if err != nil {
		return nil, err
	}

	resultData, err := json.Marshal(map[string]string{"content": string(data)})
	if err != nil {
		return nil, err
	}

	return mcp.FormatExecutionResult("read_file", "success", resultData, false), nil
}

func (s *FileSystemMCPServer) HandleWriteFile(ctx context.Context, args json.RawMessage) (*mcp.ExecutionResult, error) {
	var params struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}

	err := s.provider.WriteFile(ctx, params.Path, []byte(params.Content))
	if err != nil {
		return nil, err
	}

	resultData, err := json.Marshal(map[string]string{"status": "written", "path": params.Path})
	if err != nil {
		return nil, err
	}

	return mcp.FormatExecutionResult("write_file", "success", resultData, false), nil
}

func (s *FileSystemMCPServer) HandleListDirectory(ctx context.Context, args json.RawMessage) (*mcp.ExecutionResult, error) {
	var params struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}

	entries, err := s.provider.ListDir(ctx, params.Path)
	if err != nil {
		return nil, err
	}

	resultData, err := json.Marshal(map[string][]string{"entries": entries})
	if err != nil {
		return nil, err
	}

	return mcp.FormatExecutionResult("list_directory", "success", resultData, false), nil
}
