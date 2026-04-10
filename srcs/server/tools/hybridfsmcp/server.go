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
	Data []byte `json:"data"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *Server) ExecuteTool(ctx context.Context, toolID string, args json.RawMessage) (*mcp.ExecutionResult, error) {
	switch toolID {
	case "read_file":
		var a ReadFileArgs
		if err := json.Unmarshal(args, &a); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		data, err := s.provider.ReadFile(ctx, a.Path)
		if err != nil {
			return nil, err
		}
		resBytes, _ := json.Marshal(map[string]interface{}{"data": data})
		return mcp.FormatExecutionResult(toolID, "success", resBytes, false), nil

	case "write_file":
		var a WriteFileArgs
		if err := json.Unmarshal(args, &a); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		if err := s.provider.WriteFile(ctx, a.Path, a.Data); err != nil {
			return nil, err
		}
		resBytes, _ := json.Marshal(map[string]interface{}{"status": "written"})
		return mcp.FormatExecutionResult(toolID, "success", resBytes, false), nil

	case "list_directory":
		var a ListDirArgs
		if err := json.Unmarshal(args, &a); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		entries, err := s.provider.ListDir(ctx, a.Path)
		if err != nil {
			return nil, err
		}
		resBytes, _ := json.Marshal(map[string]interface{}{"entries": entries})
		return mcp.FormatExecutionResult(toolID, "success", resBytes, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolID)
	}
}
