package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// MCPFSHandler wraps the FileSystemProvider to handle MCP tool execution.
type MCPFSHandler struct {
	provider FileSystemProvider
}

// NewMCPFSHandler creates a new MCP handler.
func NewMCPFSHandler(provider FileSystemProvider) *MCPFSHandler {
	return &MCPFSHandler{provider: provider}
}

// ReadFileArgs defines arguments for the read_file tool.
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs defines arguments for the write_file tool.
type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 encoded or raw string depending on use-case, here we'll assume string for simplicity
}

// ListDirArgs defines arguments for the list_directory tool.
type ListDirArgs struct {
	Path string `json:"path"`
}

// ReadFileTool is the MCP tool wrapper for reading files.
func (h *MCPFSHandler) ReadFileTool(ctx context.Context, argsRaw json.RawMessage) *mcp.ExecutionResult {
	var args ReadFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(err.Error()), false)
	}

	data, err := h.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(err.Error()), false)
	}

	resData, _ := json.Marshal(map[string]string{"content": string(data)})
	return mcp.FormatExecutionResult("read_file", "success", resData, false)
}

// WriteFileTool is the MCP tool wrapper for writing files.
func (h *MCPFSHandler) WriteFileTool(ctx context.Context, argsRaw json.RawMessage) *mcp.ExecutionResult {
	var args WriteFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(err.Error()), false)
	}

	err := h.provider.WriteFile(ctx, args.Path, []byte(args.Data))
	if err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(err.Error()), false)
	}

	resData, _ := json.Marshal(map[string]string{"message": "file written successfully"})
	return mcp.FormatExecutionResult("write_file", "success", resData, false)
}

// ListDirTool is the MCP tool wrapper for listing directories.
func (h *MCPFSHandler) ListDirTool(ctx context.Context, argsRaw json.RawMessage) *mcp.ExecutionResult {
	var args ListDirArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(err.Error()), false)
	}

	infos, err := h.provider.ListDir(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(err.Error()), false)
	}

	var items []map[string]interface{}
	for _, info := range infos {
		items = append(items, map[string]interface{}{
			"name":  info.Name(),
			"size":  info.Size(),
			"isDir": info.IsDir(),
		})
	}

	resData, _ := json.Marshal(map[string]interface{}{"items": items})
	return mcp.FormatExecutionResult("list_directory", "success", resData, false)
}

// Handle routes the tool execution by ID.
func (h *MCPFSHandler) Handle(ctx context.Context, toolID string, argsRaw json.RawMessage) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		return h.ReadFileTool(ctx, argsRaw)
	case "write_file":
		return h.WriteFileTool(ctx, argsRaw)
	case "list_directory":
		return h.ListDirTool(ctx, argsRaw)
	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("unknown tool: %s", toolID)), false)
	}
}
