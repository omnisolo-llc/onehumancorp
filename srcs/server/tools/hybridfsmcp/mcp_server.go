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

type ReadFileInput struct {
	Path string `json:"path"`
}

type WriteFileInput struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirInput struct {
	Path string `json:"path"`
}

func (s *FileSystemMCPServer) HandleToolCall(ctx context.Context, toolName string, input json.RawMessage) (*mcp.ExecutionResult, error) {
	switch toolName {
	case "read_file":
		var req ReadFileInput
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for read_file: %w", err)
		}
		content, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string]string{"content": string(content)})
		return mcp.FormatExecutionResult(toolName, "success", resultData, false), nil

	case "write_file":
		var req WriteFileInput
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for write_file: %w", err)
		}
		err := s.provider.WriteFile(ctx, req.Path, []byte(req.Content))
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string]string{"message": "file written successfully"})
		return mcp.FormatExecutionResult(toolName, "success", resultData, false), nil

	case "list_directory":
		var req ListDirInput
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for list_directory: %w", err)
		}
		entries, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string][]string{"entries": entries})
		return mcp.FormatExecutionResult(toolName, "success", resultData, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
