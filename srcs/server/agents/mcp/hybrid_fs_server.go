package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	return &FileSystemMCPServer{provider: provider}
}

func (s *FileSystemMCPServer) ExecuteTool(ctx context.Context, toolID string, payload json.RawMessage) *ExecutionResult {
	var params map[string]interface{}
	if err := json.Unmarshal(payload, &params); err != nil {
		return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "invalid payload: %v"}`, err)), false)
	}

	switch toolID {
	case "read_file":
		path, _ := params["path"].(string)
		if path == "" {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "path parameter is required"}`), false)
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		return FormatExecutionResult(toolID, "success", data, false)

	case "write_file":
		path, _ := params["path"].(string)
		content, _ := params["content"].(string)
		if path == "" {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "path parameter is required"}`), false)
		}
		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		return FormatExecutionResult(toolID, "success", []byte(`{"status": "file written successfully"}`), false)

	case "list_directory":
		path, _ := params["path"].(string)
		if path == "" {
			path = "."
		}
		files, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		resultData, err := json.Marshal(map[string][]string{"files": files})
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "failed to marshal results: %v"}`, err)), false)
		}
		return FormatExecutionResult(toolID, "success", resultData, false)

	case "search_files":
		// Basic implementation: just list the directory and filter by a simple pattern for now
		path, _ := params["path"].(string)
		if path == "" {
			path = "."
		}
		pattern, _ := params["pattern"].(string)
		if pattern == "" {
			return FormatExecutionResult(toolID, "error", []byte(`{"error": "pattern parameter is required"}`), false)
		}

		files, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}

		var matchedFiles []string
		for _, file := range files {
			if strings.Contains(file, pattern) {
				matchedFiles = append(matchedFiles, file)
			}
		}

		resultData, err := json.Marshal(map[string][]string{"files": matchedFiles})
		if err != nil {
			return FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "failed to marshal results: %v"}`, err)), false)
		}
		return FormatExecutionResult(toolID, "success", resultData, false)

	default:
		return FormatExecutionResult(toolID, "error", []byte(`{"error": "unknown tool"}`), false)
	}
}
