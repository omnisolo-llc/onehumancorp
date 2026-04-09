package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
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
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *Server) HandleCall(ctx context.Context, tool string, args json.RawMessage) (interface{}, error) {
	switch tool {
	case "read_file":
		var req ReadFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}
		data, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		return string(data), nil
	case "write_file":
		var req WriteFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}
		if err := s.provider.WriteFile(ctx, req.Path, []byte(req.Content)); err != nil {
			return nil, err
		}
		return "success", nil
	case "list_directory":
		var req ListDirArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}
		return s.provider.ListDir(ctx, req.Path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", tool)
	}
}
