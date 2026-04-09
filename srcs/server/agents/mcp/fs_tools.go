package mcp

import (
	"context"
	"encoding/json"
	"fmt"
)

// ReadFileArgs represents arguments for the read_file tool.
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs represents arguments for the write_file tool.
type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// ListDirectoryArgs represents arguments for the list_directory tool.
type ListDirectoryArgs struct {
	Path string `json:"path"`
}

// FSMCPTools provides MCP tools backed by a FileSystemProvider.
type FSMCPTools struct {
	provider FileSystemProvider
}

func NewFSMCPTools(provider FileSystemProvider) *FSMCPTools {
	return &FSMCPTools{provider: provider}
}

func formatError(err error) []byte {
	res, _ := json.Marshal(map[string]string{"error": err.Error()})
	return res
}

func (t *FSMCPTools) ReadFile(ctx context.Context, argsRaw json.RawMessage) *ExecutionResult {
	var args ReadFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return FormatExecutionResult("read_file", "error", formatError(fmt.Errorf("invalid arguments: %v", err)), false)
	}

	content, err := t.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return FormatExecutionResult("read_file", "error", formatError(err), false)
	}

	resultMap := map[string]interface{}{
		"content": string(content),
	}
	resultBytes, _ := json.Marshal(resultMap)

	return FormatExecutionResult("read_file", "success", resultBytes, false)
}

func (t *FSMCPTools) WriteFile(ctx context.Context, argsRaw json.RawMessage) *ExecutionResult {
	var args WriteFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return FormatExecutionResult("write_file", "error", formatError(fmt.Errorf("invalid arguments: %v", err)), false)
	}

	err := t.provider.WriteFile(ctx, args.Path, []byte(args.Content))
	if err != nil {
		return FormatExecutionResult("write_file", "error", formatError(err), false)
	}

	resultMap := map[string]interface{}{
		"success": true,
	}
	resultBytes, _ := json.Marshal(resultMap)

	return FormatExecutionResult("write_file", "success", resultBytes, false)
}

func (t *FSMCPTools) ListDirectory(ctx context.Context, argsRaw json.RawMessage) *ExecutionResult {
	var args ListDirectoryArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return FormatExecutionResult("list_directory", "error", formatError(fmt.Errorf("invalid arguments: %v", err)), false)
	}

	files, err := t.provider.ListDir(ctx, args.Path)
	if err != nil {
		return FormatExecutionResult("list_directory", "error", formatError(err), false)
	}

	resultMap := map[string]interface{}{
		"files": files,
	}
	resultBytes, _ := json.Marshal(resultMap)

	return FormatExecutionResult("list_directory", "success", resultBytes, false)
}
