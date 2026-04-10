package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// Server is an MCP server that delegates to a FileSystemProvider.
type Server struct {
	provider FileSystemProvider
}

// NewProviderFactory instantiates the correct provider based on environment variables.
func NewProviderFactory() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider()
	}
	return NewLocalFSProvider()
}

// NewServer creates a new Hybrid FS MCP Server.
func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// Name returns the name of the MCP server.
func (s *Server) Name() string {
	return "hybridfs"
}

// Tools returns the list of tools provided by this server.
func (s *Server) Tools() []mcp.Tool {
	return []mcp.Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the given path.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file to read.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file at the given path.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file to write to.",
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
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path of the directory to list.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern in a directory (shallow search).",
			Parameters: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path of the directory to search in.",
					},
					"pattern": map[string]interface{}{
						"type":        "string",
						"description": "The string pattern to search for in file names.",
					},
				},
				"required": []string{"path", "pattern"},
			},
		},
	}
}

// Execute handles the execution of a tool.
func (s *Server) Execute(ctx context.Context, toolName string, params map[string]interface{}) (*mcp.ExecutionResult, error) {
	switch toolName {
	case "read_file":
		pathObj, ok := params["path"]
		if !ok {
			return nil, fmt.Errorf("missing path parameter")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, fmt.Errorf("path must be a string")
		}

		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}

		resBytes, _ := json.Marshal(map[string]string{"content": string(data)})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	case "write_file":
		pathObj, ok := params["path"]
		if !ok {
			return nil, fmt.Errorf("missing path parameter")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, fmt.Errorf("path must be a string")
		}

		contentObj, ok := params["content"]
		if !ok {
			return nil, fmt.Errorf("missing content parameter")
		}
		content, ok := contentObj.(string)
		if !ok {
			return nil, fmt.Errorf("content must be a string")
		}

		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}

		resBytes, _ := json.Marshal(map[string]string{"message": "success"})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	case "list_directory":
		pathObj, ok := params["path"]
		if !ok {
			return nil, fmt.Errorf("missing path parameter")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, fmt.Errorf("path must be a string")
		}

		infos, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		var files []map[string]interface{}
		for _, info := range infos {
			files = append(files, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}

		resBytes, _ := json.Marshal(map[string]interface{}{"files": files})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	case "search_files":
		pathObj, ok := params["path"]
		if !ok {
			return nil, fmt.Errorf("missing path parameter")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, fmt.Errorf("path must be a string")
		}

		patternObj, ok := params["pattern"]
		if !ok {
			return nil, fmt.Errorf("missing pattern parameter")
		}
		pattern, ok := patternObj.(string)
		if !ok {
			return nil, fmt.Errorf("pattern must be a string")
		}

		infos, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		var matches []string
		for _, info := range infos {
			// naive substring matching
			if info.Name() != "" {
				name := info.Name()
				// basic match
				if stringContains(name, pattern) {
					matches = append(matches, name)
				}
			}
		}

		resBytes, _ := json.Marshal(map[string]interface{}{"matches": matches})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func stringContains(s, substr string) bool {
	if substr == "" {
		return true
	}
	if len(s) < len(substr) {
		return false
	}
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
