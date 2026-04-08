package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

// MCP Tools wrapping the FileSystemProvider
type readDirTool struct {
	provider FileSystemProvider
}

func (t *readDirTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{
		Name:        "list_directory",
		Description: "List the contents of a directory. Returns an array of file and directory names.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Path to the directory to list.",
				},
			},
			"required": []string{"path"},
		},
	}
}

func (t *readDirTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	pathRaw, ok := input["path"]
	if !ok {
		return "", fmt.Errorf("missing required parameter: path")
	}
	path, ok := pathRaw.(string)
	if !ok {
		return "", fmt.Errorf("path must be a string")
	}

	names, err := t.provider.ListDir(ctx, path)
	if err != nil {
		return "", err
	}

	return strings.Join(names, "\n"), nil
}

type readFileTool struct {
	provider FileSystemProvider
}

func (t *readFileTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{
		Name:        "read_file",
		Description: "Read the contents of a file. Returns the file content as a string.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Path to the file.",
				},
			},
			"required": []string{"path"},
		},
	}
}

func (t *readFileTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	pathRaw, ok := input["path"]
	if !ok {
		return "", fmt.Errorf("missing required parameter: path")
	}
	path, ok := pathRaw.(string)
	if !ok {
		return "", fmt.Errorf("path must be a string")
	}

	data, err := t.provider.ReadFile(ctx, path)
	if err != nil {
		return "", err
	}

	return string(data), nil
}

type writeFileTool struct {
	provider FileSystemProvider
}

func (t *writeFileTool) Definition() local.ToolDefinition {
	return local.ToolDefinition{
		Name:        "write_file",
		Description: "Create or overwrite a file with the provided content.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"path": map[string]interface{}{
					"type":        "string",
					"description": "Path to the file.",
				},
				"content": map[string]interface{}{
					"type":        "string",
					"description": "Content to write.",
				},
			},
			"required": []string{"path", "content"},
		},
	}
}

func (t *writeFileTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	pathRaw, ok := input["path"]
	if !ok {
		return "", fmt.Errorf("missing required parameter: path")
	}
	path, ok := pathRaw.(string)
	if !ok {
		return "", fmt.Errorf("path must be a string")
	}

	contentRaw, ok := input["content"]
	if !ok {
		return "", fmt.Errorf("missing required parameter: content")
	}
	content, ok := contentRaw.(string)
	if !ok {
		// Content might have been passed as raw JSON or a struct, marshal it as fallback
		contentBytes, err := json.Marshal(contentRaw)
		if err != nil {
			return "", fmt.Errorf("content is not a string and cannot be serialized")
		}
		content = string(contentBytes)
	}

	err := t.provider.WriteFile(ctx, path, []byte(content))
	if err != nil {
		return "", err
	}

	return "Success", nil
}

// DefaultFSTools returns a list of local.Tool configured with the given provider
func DefaultFSTools(provider FileSystemProvider) []local.Tool {
	return []local.Tool{
		&readFileTool{provider: provider},
		&writeFileTool{provider: provider},
		&readDirTool{provider: provider},
	}
}
