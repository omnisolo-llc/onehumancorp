package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// NewFileSystemProvider creates a FileSystemProvider based on the current execution mode.
func NewFileSystemProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider()
	}
	return NewLocalFSProvider()
}

// HybridFSMCP is the MCP server interface for File System operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

// HandleRequest processes an incoming MCP tool execution request.
func (s *HybridFSMCP) HandleRequest(ctx context.Context, toolID string, args map[string]interface{}) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		return s.handleReadFile(ctx, args)
	case "write_file":
		return s.handleWriteFile(ctx, args)
	case "list_directory":
		return s.handleListDir(ctx, args)
	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(`{"error": "unknown tool"}`), false)
	}
}

func (s *HybridFSMCP) handleReadFile(ctx context.Context, args map[string]interface{}) *mcp.ExecutionResult {
	path, ok := args["path"].(string)
	if !ok {
		return mcp.FormatExecutionResult("read_file", "error", []byte(`{"error": "missing or invalid path parameter"}`), false)
	}

	data, err := s.provider.ReadFile(ctx, path)
	if err != nil {
		errResp, _ := json.Marshal(map[string]string{"error": err.Error()})
		return mcp.FormatExecutionResult("read_file", "error", errResp, false)
	}

	resp, _ := json.Marshal(map[string]string{"content": string(data)})
	return mcp.FormatExecutionResult("read_file", "success", resp, false)
}

func (s *HybridFSMCP) handleWriteFile(ctx context.Context, args map[string]interface{}) *mcp.ExecutionResult {
	path, ok := args["path"].(string)
	if !ok {
		return mcp.FormatExecutionResult("write_file", "error", []byte(`{"error": "missing or invalid path parameter"}`), false)
	}

	content, ok := args["content"].(string)
	if !ok {
		return mcp.FormatExecutionResult("write_file", "error", []byte(`{"error": "missing or invalid content parameter"}`), false)
	}

	err := s.provider.WriteFile(ctx, path, []byte(content))
	if err != nil {
		errResp, _ := json.Marshal(map[string]string{"error": err.Error()})
		return mcp.FormatExecutionResult("write_file", "error", errResp, false)
	}

	resp, _ := json.Marshal(map[string]bool{"success": true})
	return mcp.FormatExecutionResult("write_file", "success", resp, false)
}

func (s *HybridFSMCP) handleListDir(ctx context.Context, args map[string]interface{}) *mcp.ExecutionResult {
	path, ok := args["path"].(string)
	if !ok {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(`{"error": "missing or invalid path parameter"}`), false)
	}

	entries, err := s.provider.ListDir(ctx, path)
	if err != nil {
		errResp, _ := json.Marshal(map[string]string{"error": err.Error()})
		return mcp.FormatExecutionResult("list_directory", "error", errResp, false)
	}

	var files []string
	for _, entry := range entries {
		if entry.IsDir() {
			files = append(files, entry.Name()+"/")
		} else {
			files = append(files, entry.Name())
		}
	}

	resp, _ := json.Marshal(map[string][]string{"files": files})
	return mcp.FormatExecutionResult("list_directory", "success", resp, false)
}
