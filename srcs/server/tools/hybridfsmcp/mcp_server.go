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

func (s *FileSystemMCPServer) ExecuteTool(ctx context.Context, toolID string, arguments json.RawMessage) (*mcp.ExecutionResult, error) {
	switch toolID {
	case "read_file":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return nil, err
		}

		resData, err := json.Marshal(string(data))
		if err != nil {
			return nil, err
		}

		return mcp.FormatExecutionResult(toolID, "success", resData, false), nil

	case "write_file":
		var args struct {
			Path string `json:"path"`
			Data string `json:"data"` // Changed to string so plain text doesn't fail JSON unmarshal
		}
		if err := json.Unmarshal(arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		if err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data)); err != nil {
			return nil, err
		}
		return mcp.FormatExecutionResult(toolID, "success", []byte(`{"status": "ok"}`), false), nil

	case "list_directory":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		infos, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return nil, err
		}

		var result []map[string]interface{}
		for _, info := range infos {
			result = append(result, map[string]interface{}{
				"name": info.Name(),
				"size": info.Size(),
				"is_dir": info.IsDir(),
			})
		}
		if result == nil {
			result = []map[string]interface{}{}
		}

		resultData, err := json.Marshal(result)
		if err != nil {
			return nil, err
		}
		return mcp.FormatExecutionResult(toolID, "success", resultData, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolID)
	}
}
