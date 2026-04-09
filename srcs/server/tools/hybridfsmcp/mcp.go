package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	InputSchema interface{} `json:"inputSchema"`
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the filesystem.",
			InputSchema: map[string]interface{}{"type": "object", "properties": map[string]interface{}{"path": map[string]interface{}{"type": "string"}}},
		},
		{
			Name:        "write_file",
			Description: "Writes a file to the filesystem.",
			InputSchema: map[string]interface{}{"type": "object", "properties": map[string]interface{}{"path": map[string]interface{}{"type": "string"}, "content": map[string]interface{}{"type": "string"}}},
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory.",
			InputSchema: map[string]interface{}{"type": "object", "properties": map[string]interface{}{"path": map[string]interface{}{"type": "string"}}},
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		content, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return string(content), nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing content")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return "success", nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		return m.provider.ListDir(ctx, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
