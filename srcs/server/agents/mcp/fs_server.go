package mcp

import (
	"encoding/json"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemServer struct {
	Provider FileSystemProvider
}

func NewFileSystemServer(workspaceDir string, claims *auth.Claims, isStandalone bool) *FileSystemServer {
	var provider FileSystemProvider
	if isStandalone {
		provider = NewLocalFSProvider(workspaceDir)
	} else {
		provider = NewCloudFSProvider(workspaceDir, claims)
	}
	return &FileSystemServer{Provider: provider}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *FileSystemServer) ExecuteTool(toolName string, argsRaw json.RawMessage) *ExecutionResult {
	var result interface{}
	var err error

	switch toolName {
	case "read_file":
		var args ReadFileArgs
		if err = json.Unmarshal(argsRaw, &args); err == nil {
			var content []byte
			content, err = s.Provider.ReadFile(args.Path)
			result = map[string]string{"content": string(content)}
		}
	case "write_file":
		var args WriteFileArgs
		if err = json.Unmarshal(argsRaw, &args); err == nil {
			err = s.Provider.WriteFile(args.Path, []byte(args.Content))
			result = map[string]string{"status": "success"}
		}
	case "list_directory":
		var args ListDirArgs
		if err = json.Unmarshal(argsRaw, &args); err == nil {
			var entries []string
			entries, err = s.Provider.ListDir(args.Path)
			result = map[string][]string{"entries": entries}
		}
	default:
		err = errors.New("unknown tool")
	}

	if err != nil {
		errData, _ := json.Marshal(map[string]string{"error": err.Error()})
		return FormatExecutionResult(toolName, "error", errData, false)
	}

	resData, _ := json.Marshal(result)
	return FormatExecutionResult(toolName, "success", resData, false)
}
