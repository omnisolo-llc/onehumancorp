package hybridfs

import (
	"context"
	"fmt"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// HybridFSMCP implements an MCP server for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of filesystem tools available in this MCP.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file",
		},
		{
			Name:        "write_file",
			Description: "Write content to a file",
		},
		{
			Name:        "list_dir",
			Description: "List the contents of a directory",
		},
	}
}

// CallTool executes a tool based on the provided tool name and arguments.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(data)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}

		contentRaw, ok := arguments["content"]
		if !ok {
			return nil, fmt.Errorf("missing 'content' argument")
		}

		var content []byte
		switch v := contentRaw.(type) {
		case string:
			content = []byte(v)
		case []byte:
			content = v
		default:
			return nil, fmt.Errorf("invalid 'content' argument type: expected string or []byte")
		}

		err := m.provider.WriteFile(ctx, path, content)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"success": true}, nil

	case "list_dir":
		path, ok := arguments["path"].(string)
		if !ok {
			// default to root/base directory if not provided
			path = ""
		}

		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		// Encode to JSON string or just return the slice depending on what the typical OHC MCP does.
		// Returning map of items is usually standard.
		return map[string]interface{}{"entries": entries}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
