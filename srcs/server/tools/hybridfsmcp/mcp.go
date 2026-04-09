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
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read file contents",
			InputSchema: `{"type": "object", "properties": {"key": {"type": "string"}}}`,
		},
		{
			Name:        "write_file",
			Description: "Write file contents",
			InputSchema: `{"type": "object", "properties": {"key": {"type": "string"}, "content": {"type": "string"}}}`,
		},
		{
			Name:        "list_directory",
			Description: "List directory contents",
			InputSchema: `{"type": "object", "properties": {"prefix": {"type": "string"}}}`,
		},
		{
			Name:        "search_files",
			Description: "Search files",
			InputSchema: `{"type": "object", "properties": {"pattern": {"type": "string"}}}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		content, err := m.provider.ReadFile(ctx, key)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(content)}, nil
	case "write_file":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, key, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		prefix := ""
		if p, ok := arguments["prefix"].(string); ok {
			prefix = p
		}
		res, err := m.provider.ListDir(ctx, prefix)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "results": res}, nil
	case "search_files":
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		res, err := m.provider.SearchFiles(ctx, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "results": res}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
