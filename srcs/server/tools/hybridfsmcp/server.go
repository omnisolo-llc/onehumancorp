package hybridfsmcp

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// HybridFSServer implements the MCP server interface for file system operations.
type HybridFSServer struct {
	provider FileSystemProvider
}

// NewHybridFSServer creates a new MCP server backed by the provided FileSystemProvider.
func NewHybridFSServer(provider FileSystemProvider) *HybridFSServer {
	return &HybridFSServer{
		provider: provider,
	}
}

// ReadFileTool executes the read_file tool.
func (s *HybridFSServer) ReadFileTool(ctx context.Context, path string) *mcp.ExecutionResult {
	content, err := s.provider.ReadFile(ctx, path)
	if err != nil {
		return s.errorResult("read_file", err)
	}

	resultData, _ := json.Marshal(map[string]string{
		"path":    path,
		"content": string(content),
	})

	return mcp.FormatExecutionResult("read_file", "success", resultData, false)
}

// WriteFileTool executes the write_file tool.
func (s *HybridFSServer) WriteFileTool(ctx context.Context, path string, content string) *mcp.ExecutionResult {
	err := s.provider.WriteFile(ctx, path, []byte(content))
	if err != nil {
		return s.errorResult("write_file", err)
	}

	resultData, _ := json.Marshal(map[string]string{
		"path":    path,
		"message": "file written successfully",
	})

	return mcp.FormatExecutionResult("write_file", "success", resultData, false)
}

// ListDirectoryTool executes the list_directory tool.
func (s *HybridFSServer) ListDirectoryTool(ctx context.Context, path string) *mcp.ExecutionResult {
	entries, err := s.provider.ListDir(ctx, path)
	if err != nil {
		return s.errorResult("list_directory", err)
	}

	var items []map[string]interface{}
	for _, entry := range entries {
		items = append(items, map[string]interface{}{
			"name":  entry.Name(),
			"isdir": entry.IsDir(),
		})
	}

	resultData, _ := json.Marshal(map[string]interface{}{
		"path":    path,
		"entries": items,
	})

	return mcp.FormatExecutionResult("list_directory", "success", resultData, false)
}

// SearchFilesTool executes the search_files tool.
func (s *HybridFSServer) SearchFilesTool(ctx context.Context, path, query string) *mcp.ExecutionResult {
	files, err := s.provider.SearchFiles(ctx, path, query)
	if err != nil {
		return s.errorResult("search_files", err)
	}

	resultData, _ := json.Marshal(map[string]interface{}{
		"path":    path,
		"query":   query,
		"matches": files,
	})

	return mcp.FormatExecutionResult("search_files", "success", resultData, false)
}

// errorResult returns an execution result indicating an error.
func (s *HybridFSServer) errorResult(toolID string, err error) *mcp.ExecutionResult {
	resultData, _ := json.Marshal(map[string]string{
		"error": err.Error(),
	})
	return &mcp.ExecutionResult{
		ToolID:           toolID,
		Status:           "error",
		ResultData:       resultData,
		HybridEscalation: false,
		Escalation:       false,
		ExecutedAt:       time.Now().UTC(),
	}
}
