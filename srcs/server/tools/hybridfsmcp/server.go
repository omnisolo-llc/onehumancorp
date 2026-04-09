package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// Server implements an MCP server for file system operations.
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new Server.
func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// NewHybridFSProvider is a factory that returns the appropriate provider depending on mode.
func NewHybridFSProvider(workspaceRoot string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(workspaceRoot)
	}
	// Default to cloud provider for multi-tenant deployments
	return NewCloudFSProvider(workspaceRoot)
}

// CallTool executes a tool on the MCP server.
func (s *Server) CallTool(ctx context.Context, name string, arguments map[string]interface{}) (interface{}, error) {
	switch name {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, fmt.Errorf("read file failed: %w", err)
		}
		return map[string]interface{}{
			"content": string(data),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: content")
		}
		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, fmt.Errorf("write file failed: %w", err)
		}
		return map[string]interface{}{
			"success": true,
		}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}
		entries, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, fmt.Errorf("list directory failed: %w", err)
		}
		var files []string
		var dirs []string
		for _, e := range entries {
			if e.IsDir() {
				dirs = append(dirs, e.Name())
			} else {
				files = append(files, e.Name())
			}
		}
		return map[string]interface{}{
			"files": files,
			"directories": dirs,
		}, nil

	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: pattern")
		}
		results, err := s.provider.SearchFiles(ctx, path, pattern)
		if err != nil {
			return nil, fmt.Errorf("search files failed: %w", err)
		}
		return map[string]interface{}{
			"results": results,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

// GetToolDescription returns the JSON schema definition for the exposed tools.
func (s *Server) GetToolDescription() []byte {
	description := []map[string]interface{}{
		{
			"name": "read_file",
			"description": "Read the contents of a file.",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type": "string",
						"description": "The path to the file.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name": "write_file",
			"description": "Write content to a file. Overwrites if it exists.",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type": "string",
						"description": "The path to the file.",
					},
					"content": map[string]interface{}{
						"type": "string",
						"description": "The content to write.",
					},
				},
				"required": []string{"path", "content"},
			},
		},
		{
			"name": "list_directory",
			"description": "List files and directories in a given path.",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type": "string",
						"description": "The path to the directory.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name": "search_files",
			"description": "Search for files matching a pattern in a given path.",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type": "string",
						"description": "The root path to search from.",
					},
					"pattern": map[string]interface{}{
						"type": "string",
						"description": "The file glob pattern to match (e.g., *.txt).",
					},
				},
				"required": []string{"path", "pattern"},
			},
		},
	}

	bytes, _ := json.Marshal(description)
	return bytes
}
