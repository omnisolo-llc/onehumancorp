package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// Server represents the Hybrid File System MCP server.
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new Hybrid File System MCP server based on the environment.
func NewServer(baseDir string) (*Server, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider, err = NewCloudFSProvider(baseDir)
	} else {
		provider, err = NewLocalFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &Server{provider: provider}, nil
}

// ExecuteTool implements the generic MCP tool execution interface.
func (s *Server) ExecuteTool(ctx context.Context, toolName string, input map[string]interface{}) *mcp.ExecutionResult {
	path, ok := input["path"].(string)
	if !ok {
		return s.errorResult(toolName, "missing or invalid 'path' argument")
	}

	var data []byte
	var err error

	switch toolName {
	case "read_file":
		data, err = s.provider.ReadFile(ctx, path)
	case "write_file":
		contentStr, ok := input["content"].(string)
		if !ok {
			return s.errorResult(toolName, "missing or invalid 'content' argument")
		}
		err = s.provider.WriteFile(ctx, path, []byte(contentStr), 0644)
		if err == nil {
			data = []byte(`{"status":"success"}`)
		}
	case "list_directory":
		entries, err := s.provider.ListDir(ctx, path)
		if err == nil {
			var names []string
			for _, e := range entries {
				names = append(names, e.Name())
			}
			data, err = json.Marshal(names)
		}
	default:
		return s.errorResult(toolName, fmt.Sprintf("unknown tool: %s", toolName))
	}

	if err != nil {
		return s.errorResult(toolName, err.Error())
	}

	return mcp.FormatExecutionResult(toolName, "success", data, false)
}

func (s *Server) errorResult(toolName, msg string) *mcp.ExecutionResult {
	data, _ := json.Marshal(map[string]string{"error": msg})
	return mcp.FormatExecutionResult(toolName, "error", data, false)
}
