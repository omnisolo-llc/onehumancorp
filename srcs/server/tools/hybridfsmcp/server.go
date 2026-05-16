package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
)

type tenantIDKey struct{}

type Server struct {
	provider FileSystemProvider
}

type Claims struct {
	OrganizationID string
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

func (s *Server) ListTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "read_file",
			"description": "Reads a file from the file system.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name":        "write_file",
			"description": "Writes content to a file in the file system.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file.",
					},
					"content": map[string]interface{}{
						"type":        "string",
						"description": "The content to write.",
					},
				},
				"required": []string{"path", "content"},
			},
		},
		{
			"name":        "list_directory",
			"description": "Lists files and directories in a given path.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the directory.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name":        "search_files",
			"description": "Searches for files matching a query string in the filename.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The directory path to search in.",
					},
					"query": map[string]interface{}{
						"type":        "string",
						"description": "The query string to match against filenames.",
					},
				},
				"required": []string{"path", "query"},
			},
		},
	}
}

func (s *Server) CallTool(ctx context.Context, name string, args map[string]interface{}, claims *Claims) (interface{}, error) {
	if claims == nil {
		return nil, errors.New("unauthorized: claims are missing")
	}

	if s.provider == nil {
		return nil, errors.New("file system provider not configured")
	}

	// For cloud provider, we need to pass tenant_id via context.
	// Since we know Claims struct has OrganizationID, we use it as tenant_id.
	if !s.provider.IsLocal() {
		ctx = context.WithValue(ctx, tenantIDKey{}, claims.OrganizationID)
	}

	switch name {
	case "read_file":
		pathStr, ok := args["path"].(string)
		if !ok || pathStr == "" {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		data, err := s.provider.ReadFile(ctx, pathStr)
		if err != nil {
			return nil, fmt.Errorf("failed to read file: %w", err)
		}
		return map[string]interface{}{
			"content": string(data),
		}, nil

	case "write_file":
		pathStr, ok := args["path"].(string)
		if !ok || pathStr == "" {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		contentStr, ok := args["content"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'content' argument")
		}

		err := s.provider.WriteFile(ctx, pathStr, []byte(contentStr))
		if err != nil {
			return nil, fmt.Errorf("failed to write file: %w", err)
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		pathStr, ok := args["path"].(string)
		if !ok || pathStr == "" {
			return nil, errors.New("invalid or missing 'path' argument")
		}

		entries, err := s.provider.ListDir(ctx, pathStr)
		if err != nil {
			return nil, fmt.Errorf("failed to list directory: %w", err)
		}
		return entries, nil

	case "search_files":
		pathStr, ok := args["path"].(string)
		if !ok || pathStr == "" {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		queryStr, ok := args["query"].(string)
		if !ok || queryStr == "" {
			return nil, errors.New("invalid or missing 'query' argument")
		}

		entries, err := s.provider.SearchFiles(ctx, queryStr, pathStr)
		if err != nil {
			return nil, fmt.Errorf("failed to search files: %w", err)
		}
		return entries, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
