package mcp

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FSMCPServer is an MCP Server exposing file system operations.
type FSMCPServer struct {
	provider FileSystemProvider
}

// NewFSMCPServer creates a new FSMCPServer.
func NewFSMCPServer(provider FileSystemProvider) *FSMCPServer {
	return &FSMCPServer{
		provider: provider,
	}
}

// CallTool executes the specified filesystem tool.
func (s *FSMCPServer) CallTool(ctx context.Context, claims *auth.Claims, toolID string, input map[string]interface{}) *ExecutionResult {
	var err error
	var resultData []byte

	switch toolID {
	case "read_file":
		path, ok := input["path"].(string)
		if !ok {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "path is required"}`), false)
		}
		data, readErr := s.provider.ReadFile(ctx, claims, path)
		if readErr != nil {
			err = readErr
		} else {
			resultData, _ = json.Marshal(map[string]string{"content": string(data)})
		}

	case "write_file":
		path, ok := input["path"].(string)
		if !ok {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "path is required"}`), false)
		}

		var data []byte
		switch v := input["data"].(type) {
		case string:
			data = []byte(v)
		default:
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "data string is required"}`), false)
		}

		writeErr := s.provider.WriteFile(ctx, claims, path, data)
		if writeErr != nil {
			err = writeErr
		} else {
			resultData, _ = json.Marshal(map[string]string{"status": "success"})
		}

	case "list_directory":
		path, ok := input["path"].(string)
		if !ok {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "path is required"}`), false)
		}

		entries, listErr := s.provider.ListDir(ctx, claims, path)
		if listErr != nil {
			err = listErr
		} else {
			var names []string
			for _, entry := range entries {
				names = append(names, entry.Name())
			}
			resultData, _ = json.Marshal(map[string][]string{"files": names})
		}

	default:
		return FormatExecutionResult(toolID, "error", []byte(`{"error": "unknown tool"}`), false)
	}

	if err != nil {
		errBytes, _ := json.Marshal(map[string]string{"error": err.Error()})
		return FormatExecutionResult(toolID, "error", errBytes, false)
	}

	return FormatExecutionResult(toolID, "success", resultData, false)
}
