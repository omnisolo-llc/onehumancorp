package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// FSServer encapsulates a FileSystemProvider and exposes MCP-compatible tools.
type FSServer struct {
	provider FileSystemProvider
}

// NewFileSystemProvider is a factory that instantiates the appropriate
// FileSystemProvider based on the operating mode.
func NewFileSystemProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

// NewFSServer creates a new MCP Server for file system operations.
func NewFSServer(provider FileSystemProvider) *FSServer {
	return &FSServer{
		provider: provider,
	}
}

// ReadFileTool executes the read_file tool request.
func (s *FSServer) ReadFileTool(ctx context.Context, input []byte) (*ExecutionResult, error) {
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input format: %w", err)
	}

	content, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return nil, fmt.Errorf("failed to read file: %w", err)
	}

	resultData, _ := json.Marshal(map[string]string{
		"content": string(content),
	})

	return FormatExecutionResult("read_file", "success", resultData, false), nil
}

// WriteFileTool executes the write_file tool request.
func (s *FSServer) WriteFileTool(ctx context.Context, input []byte) (*ExecutionResult, error) {
	var args struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input format: %w", err)
	}

	err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
	if err != nil {
		return nil, fmt.Errorf("failed to write file: %w", err)
	}

	resultData, _ := json.Marshal(map[string]string{
		"message": "file written successfully",
	})

	return FormatExecutionResult("write_file", "success", resultData, false), nil
}

// ListDirTool executes the list_directory tool request.
func (s *FSServer) ListDirTool(ctx context.Context, input []byte) (*ExecutionResult, error) {
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(input, &args); err != nil {
		return nil, fmt.Errorf("invalid input format: %w", err)
	}

	// Use root dir if path is empty
	targetPath := args.Path
	if targetPath == "" {
		targetPath = "."
	}

	entries, err := s.provider.ListDir(ctx, targetPath)
	if err != nil {
		return nil, fmt.Errorf("failed to list directory: %w", err)
	}

	resultData, _ := json.Marshal(map[string]interface{}{
		"entries": entries,
	})

	return FormatExecutionResult("list_directory", "success", resultData, false), nil
}
