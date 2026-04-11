package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// NewFileSystemProvider creates the correct FileSystemProvider based on OHC_MULTITENANT.
func NewFileSystemProvider() FileSystemProvider {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

// Server acts as the MCP Server wrapping the FileSystemProvider.
type Server struct {
	Provider FileSystemProvider
}

// NewServer initializes the Server with the given provider, or creates the default one.
func NewServer(provider FileSystemProvider) *Server {
	if provider == nil {
		provider = NewFileSystemProvider()
	}
	return &Server{Provider: provider}
}

// HandleToolCall executes the corresponding filesystem operation.
func (s *Server) HandleToolCall(ctx context.Context, toolID string, input map[string]interface{}) *mcp.ExecutionResult {
	pathVal, ok := input["path"].(string)
	if !ok {
		return mcp.FormatExecutionResult(toolID, "error", []byte("missing or invalid 'path' argument"), false)
	}

	switch toolID {
	case "read_file":
		data, err := s.Provider.ReadFile(ctx, pathVal)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", data, false)

	case "write_file":
		contentVal, ok := input["content"].(string)
		if !ok {
			return mcp.FormatExecutionResult(toolID, "error", []byte("missing or invalid 'content' argument"), false)
		}
		err := s.Provider.WriteFile(ctx, pathVal, []byte(contentVal))
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", []byte(fmt.Sprintf("successfully wrote to %s", pathVal)), false)

	case "list_directory":
		entries, err := s.Provider.ListDir(ctx, pathVal)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}

		var fileNames []string
		for _, e := range entries {
			fileNames = append(fileNames, e.Name())
		}

		data, err := json.Marshal(fileNames)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", data, false)

	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("unknown tool %s", toolID)), false)
	}
}
