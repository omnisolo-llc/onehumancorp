package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

// MCP server that exposes tools to work with a hybrid filesystem.
type FSServer struct {
	provider FileSystemProvider
}

// ReadFileArgs are the arguments for the read_file tool
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs are the arguments for the write_file tool
type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// ListDirArgs are the arguments for the list_directory tool
type ListDirArgs struct {
	Path string `json:"path"`
}

func NewFSServer(provider FileSystemProvider) *FSServer {
	return &FSServer{provider: provider}
}

func (s *FSServer) GetTools() []local.ToolDefinition {
	return []local.ToolDefinition{
		{
			Name:        "read_file",
			Description: "Reads the content of a file at the specified path.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The relative path to the file to read.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file at the specified path.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The relative path to the file to write.",
					},
					"content": map[string]interface{}{
						"type":        "string", // IMPORTANT: memory rule, mcp tool argument structs receiving plain text must be string, not []byte
						"description": "The text content to write to the file.",
					},
				},
				"required": []string{"path", "content"},
			},
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories at the specified path.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The relative path of the directory to list.",
					},
				},
				"required": []string{"path"},
			},
		},
	}
}

func (s *FSServer) CallTool(ctx context.Context, name string, argsRaw []byte) (string, error) {
	switch name {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return "", fmt.Errorf("failed to parse read_file args: %v", err)
		}
		return s.provider.ReadFile(ctx, args.Path)

	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return "", fmt.Errorf("failed to parse write_file args: %v", err)
		}
		err := s.provider.WriteFile(ctx, args.Path, args.Content)
		if err != nil {
			return "", err
		}
		return "Successfully wrote to file.", nil

	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(argsRaw, &args); err != nil {
			return "", fmt.Errorf("failed to parse list_directory args: %v", err)
		}
		infos, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return "", err
		}

		var result string
		for _, info := range infos {
			itemType := "file"
			if info.IsDir() {
				itemType = "dir "
			}
			result += fmt.Sprintf("[%s] %s\n", itemType, info.Name())
		}
		if result == "" {
			return "Directory is empty.", nil
		}
		return result, nil

	default:
		return "", fmt.Errorf("unknown tool: %s", name)
	}
}

// Factory logic
func NewProviderFromEnv() (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseDir := os.Getenv("OHC_CLOUD_FS_BASE")
		if baseDir == "" {
			baseDir = "/mnt/k8s/tenant-volumes"
		}
		return NewCloudFSProvider(baseDir)
	}

	// Standalone default
	baseDir := os.Getenv("OHC_STANDALONE_FS_BASE")
	if baseDir == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, fmt.Errorf("failed to get working dir: %v", err)
		}
		baseDir = cwd
	}
	return NewLocalFSProvider(baseDir)
}
