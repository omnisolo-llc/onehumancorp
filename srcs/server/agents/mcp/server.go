package mcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type FileSystemServer struct {
	provider FileSystemProvider
}

func NewFileSystemServer(provider FileSystemProvider) *FileSystemServer {
	return &FileSystemServer{provider: provider}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *FileSystemServer) HandleToolCall(ctx context.Context, toolName string, argsRaw json.RawMessage) *ExecutionResult {
	switch toolName {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.errorResult(toolName, err)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return s.errorResult(toolName, err)
		}
		resBytes, _ := json.Marshal(map[string]string{"content": string(data)})
		return FormatExecutionResult(toolName, "success", resBytes, false)

	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.errorResult(toolName, err)
		}
		if err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data)); err != nil {
			return s.errorResult(toolName, err)
		}
		resBytes, _ := json.Marshal(map[string]bool{"success": true})
		return FormatExecutionResult(toolName, "success", resBytes, false)

	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return s.errorResult(toolName, err)
		}
		entries, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return s.errorResult(toolName, err)
		}
		resBytes, _ := json.Marshal(map[string][]string{"entries": entries})
		return FormatExecutionResult(toolName, "success", resBytes, false)

	default:
		return s.errorResult(toolName, fmt.Errorf("unknown tool: %s", toolName))
	}
}

func (s *FileSystemServer) errorResult(toolName string, err error) *ExecutionResult {
	resBytes, _ := json.Marshal(map[string]string{"error": err.Error()})
	return FormatExecutionResult(toolName, "error", resBytes, false)
}
