package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// HybridFSServer exposes filesystem tools via MCP.
type HybridFSServer struct {
	provider mcp.FileSystemProvider
}

// NewServer creates a new HybridFSServer, auto-selecting the provider
// based on the OHC_MULTITENANT environment variable.
func NewServer(ctx context.Context) (*HybridFSServer, error) {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}

	isCloud := os.Getenv("OHC_MULTITENANT") == "true"
	var provider mcp.FileSystemProvider
	if isCloud {
		provider = NewCloudFSProvider(baseDir)
	} else {
		provider = NewLocalFSProvider(baseDir)
	}

	return &HybridFSServer{provider: provider}, nil
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"` // Base64 or plain string depending on encoding; assuming plain string for simplicity here
}

type ListDirArgs struct {
	Path string `json:"path"`
}

// CallTool implements the standard MCP tool execution entrypoint.
func (s *HybridFSServer) CallTool(ctx context.Context, toolID string, input json.RawMessage) *mcp.ExecutionResult {
	switch toolID {
	case "read_file":
		return s.handleReadFile(ctx, input)
	case "write_file":
		return s.handleWriteFile(ctx, input)
	case "list_directory":
		return s.handleListDir(ctx, input)
	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf("unknown tool: %s", toolID)), false)
	}
}

func (s *HybridFSServer) handleReadFile(ctx context.Context, input json.RawMessage) *mcp.ExecutionResult {
	var args ReadFileArgs
	if err := json.Unmarshal(input, &args); err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(err.Error()), false)
	}

	data, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(err.Error()), false)
	}

	return mcp.FormatExecutionResult("read_file", "success", data, false)
}

func (s *HybridFSServer) handleWriteFile(ctx context.Context, input json.RawMessage) *mcp.ExecutionResult {
	var args WriteFileArgs
	if err := json.Unmarshal(input, &args); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(err.Error()), false)
	}

	err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content), 0644)
	if err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(err.Error()), false)
	}

	return mcp.FormatExecutionResult("write_file", "success", []byte("file written successfully"), false)
}

func (s *HybridFSServer) handleListDir(ctx context.Context, input json.RawMessage) *mcp.ExecutionResult {
	var args ListDirArgs
	if err := json.Unmarshal(input, &args); err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(err.Error()), false)
	}

	infos, err := s.provider.ListDir(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(err.Error()), false)
	}

	names := []string{}
	for _, info := range infos {
		names = append(names, info.Name())
	}

	data, err := json.Marshal(names)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(err.Error()), false)
	}

	return mcp.FormatExecutionResult("list_directory", "success", data, false)
}
