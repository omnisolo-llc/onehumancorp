package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer() (*Server, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseDir := os.Getenv("OHC_CLOUD_FS_BASE")
		if baseDir == "" {
			baseDir = "/var/lib/ohc/cloudfs"
		}
		provider, err = NewCloudFSProvider(baseDir)
	} else {
		workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
		if workspaceDir == "" {
			workspaceDir = "./workspace"
		}
		provider, err = NewLocalFSProvider(workspaceDir)
	}

	if err != nil {
		return nil, err
	}

	return &Server{provider: provider}, nil
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 encoded or raw string, for simplicity raw string here
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *Server) ExecuteTool(ctx context.Context, toolID string, rawArgs json.RawMessage) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("invalid args: %v", err)), false)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", data, false)
	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("invalid args: %v", err)), false)
		}
		err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data))
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		return mcp.FormatExecutionResult(toolID, "success", []byte(`{"success": true}`), false)
	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("invalid args: %v", err)), false)
		}
		entries, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false)
		}
		entriesJSON, _ := json.Marshal(entries)
		return mcp.FormatExecutionResult(toolID, "success", entriesJSON, false)
	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("unknown tool: %s", toolID)), false)
	}
}
