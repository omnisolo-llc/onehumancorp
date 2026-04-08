package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Server struct {
	baseDir string
	isStandalone bool
}

func NewServer(baseDir string) *Server {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	return &Server{
		baseDir: baseDir,
		isStandalone: isStandalone,
	}
}

// ListTools returns the tools supported by this MCP server.
func (s *Server) ListTools(ctx context.Context) ([]map[string]interface{}, error) {
	return []map[string]interface{}{
		{
			"name":        "read_file",
			"description": "Reads the content of a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"filepath": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file to read",
					},
				},
				"required": []string{"filepath"},
			},
		},
		{
			"name":        "write_file",
			"description": "Writes content to a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"filepath": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file to write",
					},
					"content": map[string]interface{}{
						"type":        "string",
						"description": "Content to write",
					},
				},
				"required": []string{"filepath", "content"},
			},
		},
		{
			"name":        "list_directory",
			"description": "Lists files in a directory",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"dirpath": map[string]interface{}{
						"type":        "string",
						"description": "Path to the directory to list",
					},
				},
				"required": []string{"dirpath"},
			},
		},
		{
			"name":        "search_files",
			"description": "Searches for files matching a pattern (unimplemented stub)",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"pattern": map[string]interface{}{
						"type":        "string",
						"description": "Pattern to search for",
					},
				},
				"required": []string{"pattern"},
			},
		},
	}, nil
}

func (s *Server) getProvider(ctx context.Context) (FileSystemProvider, error) {
	claims := auth.ClaimsFromContext(ctx)
	organizationID := ""
	if claims != nil {
		organizationID = claims.OrganizationID
	}

	if !s.isStandalone && organizationID == "" {
		return nil, errors.New("organization ID is required in Cloud mode")
	}

	return NewFileSystemProvider(s.isStandalone, s.baseDir, organizationID)
}

// CallTool executes a tool call.
func (s *Server) CallTool(ctx context.Context, name string, arguments map[string]interface{}) (interface{}, error) {
	provider, err := s.getProvider(ctx)
	if err != nil {
		return nil, err
	}

	switch name {
	case "read_file":
		filepath, ok := arguments["filepath"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'filepath' argument")
		}

		content, err := provider.ReadFile(filepath)
		if err != nil {
			return nil, err
		}
		return string(content), nil

	case "write_file":
		filepath, ok := arguments["filepath"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'filepath' argument")
		}

		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'content' argument")
		}

		if err := provider.WriteFile(filepath, []byte(content)); err != nil {
			return nil, err
		}
		return "File written successfully", nil

	case "list_directory":
		dirpath, ok := arguments["dirpath"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'dirpath' argument")
		}

		files, err := provider.ListDir(dirpath)
		if err != nil {
			return nil, err
		}
		return files, nil

	case "search_files":
		// unimplemented stub to satisfy interface
		return []string{}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
