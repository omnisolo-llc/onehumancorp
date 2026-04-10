package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// HybridFSServer wraps a FileSystemProvider to expose MCP tools.
type HybridFSServer struct {
	provider FileSystemProvider
}

// NewHybridFSServer creates a new HybridFSServer.
func NewHybridFSServer(provider FileSystemProvider) *HybridFSServer {
	return &HybridFSServer{
		provider: provider,
	}
}

// ReadFileArgs represents arguments for read_file tool.
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs represents arguments for write_file tool.
type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

// ListDirArgs represents arguments for list_directory tool.
type ListDirArgs struct {
	Path string `json:"path"`
}

// Call executes the requested tool.
func (s *HybridFSServer) Call(ctx context.Context, toolID string, rawArgs []byte) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		resBytes, _ := json.Marshal(map[string]string{"content": string(data)})
		return mcp.FormatExecutionResult(toolID, "success", resBytes, false)

	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data))
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", []byte(`{"status": "written"}`), false)

	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		names, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
		}
		resBytes, _ := json.Marshal(map[string][]string{"files": names})
		return mcp.FormatExecutionResult(toolID, "success", resBytes, false)

	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(`{"error": "unknown tool"}`), false)
	}
}
