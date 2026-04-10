package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

func (s *Server) ListTools(ctx context.Context) ([]string, error) {
	return []string{
		"read_file",
		"write_file",
		"list_directory",
		"search_files",
	}, nil
}

type CallToolRequest struct {
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments"`
}

func (s *Server) CallTool(ctx context.Context, req CallToolRequest) (*mcp.ExecutionResult, error) {
	var resultData []byte
	var err error

	switch req.Name {
	case "read_file":
		var args struct {
			Path string `json:"path"`
		}
		if err = json.Unmarshal(req.Arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}

		content, readErr := s.provider.ReadFile(ctx, args.Path)
		if readErr != nil {
			return mcp.FormatExecutionResult(req.Name, "error", []byte(readErr.Error()), false), nil
		}

		resultData, err = json.Marshal(map[string]string{"content": string(content)})

	case "write_file":
		var args struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err = json.Unmarshal(req.Arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}

		writeErr := s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
		if writeErr != nil {
			return mcp.FormatExecutionResult(req.Name, "error", []byte(writeErr.Error()), false), nil
		}

		resultData, err = json.Marshal(map[string]string{"status": "success"})

	case "list_directory":
		var args struct {
			Path string `json:"path"`
		}
		if err = json.Unmarshal(req.Arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}

		entries, listErr := s.provider.ListDir(ctx, args.Path)
		if listErr != nil {
			return mcp.FormatExecutionResult(req.Name, "error", []byte(listErr.Error()), false), nil
		}

		resultData, err = json.Marshal(map[string][]string{"entries": entries})

	case "search_files":
		var args struct {
			Dir     string `json:"dir"`
			Pattern string `json:"pattern"`
		}
		if err = json.Unmarshal(req.Arguments, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}

		// Ensure path is cleaned
		dir := filepath.Clean(args.Dir)

		matches, searchErr := s.provider.SearchFiles(ctx, dir, args.Pattern)
		if searchErr != nil {
			return mcp.FormatExecutionResult(req.Name, "error", []byte(searchErr.Error()), false), nil
		}

		resultData, err = json.Marshal(map[string][]string{"matches": matches})

	default:
		return nil, fmt.Errorf("unknown tool: %s", req.Name)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to marshal result: %w", err)
	}

	return mcp.FormatExecutionResult(req.Name, "success", resultData, false), nil
}
