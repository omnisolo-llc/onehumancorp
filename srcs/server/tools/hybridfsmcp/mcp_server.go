package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// ReadFileInput is the parameter schema for the read_file tool.
type ReadFileInput struct {
	Path string `json:"path"`
}

// WriteFileInput is the parameter schema for the write_file tool.
type WriteFileInput struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

// ListDirInput is the parameter schema for the list_directory tool.
type ListDirInput struct {
	Path string `json:"path"`
}

// SearchFilesInput is the parameter schema for the search_files tool.
type SearchFilesInput struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

// HybridFSMCPServer wraps a FileSystemProvider to expose MCP tools.
type HybridFSMCPServer struct {
	provider FileSystemProvider
}

// NewHybridFSMCPServer creates a new server with the given provider.
func NewHybridFSMCPServer(provider FileSystemProvider) *HybridFSMCPServer {
	return &HybridFSMCPServer{
		provider: provider,
	}
}

// CallTool executes the requested tool.
func (s *HybridFSMCPServer) CallTool(ctx context.Context, claims *auth.Claims, toolName string, input json.RawMessage) (interface{}, error) {
	switch toolName {
	case "read_file":
		var args ReadFileInput
		if err := json.Unmarshal(input, &args); err != nil {
			return nil, fmt.Errorf("invalid input for read_file: %w", err)
		}
		data, err := s.provider.ReadFile(ctx, claims, args.Path)
		if err != nil {
			return nil, err
		}
		return string(data), nil

	case "write_file":
		var args WriteFileInput
		if err := json.Unmarshal(input, &args); err != nil {
			return nil, fmt.Errorf("invalid input for write_file: %w", err)
		}
		err := s.provider.WriteFile(ctx, claims, args.Path, []byte(args.Data))
		if err != nil {
			return nil, err
		}
		return map[string]string{"status": "success"}, nil

	case "list_directory":
		var args ListDirInput
		if err := json.Unmarshal(input, &args); err != nil {
			return nil, fmt.Errorf("invalid input for list_directory: %w", err)
		}

		// If path is empty, default to root "."
		path := args.Path
		if path == "" {
			path = "."
		}

		return s.provider.ListDir(ctx, claims, path)

	case "search_files":
		var args SearchFilesInput
		if err := json.Unmarshal(input, &args); err != nil {
			return nil, fmt.Errorf("invalid input for search_files: %w", err)
		}

		path := args.Path
		if path == "" {
			path = "."
		}

		return s.provider.SearchFiles(ctx, claims, path, args.Pattern)

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
