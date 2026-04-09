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
	Path string `json:"path"`
	Data string `json:"data"`
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
		return map[string]string{"content": string(data)}, nil
	case "write_file":
		var req WriteFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}
		if err := s.provider.WriteFile(ctx, req.Path, []byte(req.Data)); err != nil {
			return nil, err
		}
		return map[string]string{"status": "success"}, nil
	case "list_directory":
		var req ListDirArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}
		entries, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}

		var result []string
		for _, e := range entries {
			// Memory rule: when iterating over os.ReadDir entries and calling e.Info(),
			// always check for the returned error
			info, err := e.Info()
			if err != nil {
				continue // Skip if we can't get info
			}
			result = append(result, fmt.Sprintf("%s (%d bytes)", e.Name(), info.Size()))
		}
		return result, nil
	default:
		return nil, fmt.Errorf("unknown tool %s", tool)
	}
}
