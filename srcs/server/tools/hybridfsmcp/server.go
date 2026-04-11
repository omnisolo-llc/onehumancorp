package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// Server encapsulates the MCP server logic for filesystem operations.
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new HybridFS MCP Server.
func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// ReadFileArgs represents the arguments for the read_file tool.
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs represents the arguments for the write_file tool.
type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// ListDirectoryArgs represents the arguments for the list_directory tool.
type ListDirectoryArgs struct {
	Path string `json:"path"`
}

// SearchFilesArgs represents the arguments for the search_files tool.
type SearchFilesArgs struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

// ExecuteTool handles the invocation of standard filesystem tools.
func (s *Server) ExecuteTool(ctx context.Context, toolName string, args json.RawMessage) *mcp.ExecutionResult {
	switch toolName {
	case "read_file":
		return s.executeReadFile(ctx, args)
	case "write_file":
		return s.executeWriteFile(ctx, args)
	case "list_directory":
		return s.executeListDirectory(ctx, args)
	case "search_files":
		return s.executeSearchFiles(ctx, args)
	default:
		return mcp.FormatExecutionResult(toolName, "error", []byte("unknown tool"), false)
	}
}

func (s *Server) executeReadFile(ctx context.Context, rawArgs json.RawMessage) *mcp.ExecutionResult {
	var args ReadFileArgs
	if err := json.Unmarshal(rawArgs, &args); err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	data, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf("read failed: %v", err)), false)
	}

	// Assuming data is text for this example, or we could return base64
	resultData, _ := json.Marshal(map[string]string{"content": string(data)})
	return mcp.FormatExecutionResult("read_file", "success", resultData, false)
}

func (s *Server) executeWriteFile(ctx context.Context, rawArgs json.RawMessage) *mcp.ExecutionResult {
	var args WriteFileArgs
	if err := json.Unmarshal(rawArgs, &args); err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
	if err != nil {
		return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf("write failed: %v", err)), false)
	}

	resultData, _ := json.Marshal(map[string]string{"message": "file written successfully"})
	return mcp.FormatExecutionResult("write_file", "success", resultData, false)
}

func (s *Server) executeListDirectory(ctx context.Context, rawArgs json.RawMessage) *mcp.ExecutionResult {
	var args ListDirectoryArgs
	if err := json.Unmarshal(rawArgs, &args); err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	files, err := s.provider.ListDir(ctx, args.Path)
	if err != nil {
		return mcp.FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf("list directory failed: %v", err)), false)
	}

	resultData, _ := json.Marshal(map[string][]string{"files": files})
	return mcp.FormatExecutionResult("list_directory", "success", resultData, false)
}

func (s *Server) executeSearchFiles(ctx context.Context, rawArgs json.RawMessage) *mcp.ExecutionResult {
	var args SearchFilesArgs
	if err := json.Unmarshal(rawArgs, &args); err != nil {
		return mcp.FormatExecutionResult("search_files", "error", []byte(fmt.Sprintf("invalid arguments: %v", err)), false)
	}

	files, err := s.provider.SearchFiles(ctx, args.Path, args.Pattern)
	if err != nil {
		return mcp.FormatExecutionResult("search_files", "error", []byte(fmt.Sprintf("search files failed: %v", err)), false)
	}

	resultData, _ := json.Marshal(map[string][]string{"files": files})
	return mcp.FormatExecutionResult("search_files", "success", resultData, false)
}
