package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

// FSInspectorMCP implements the MCP interface for file system operations.
type FSInspectorMCP struct {
	provider FileSystemProvider
}

// NewFSInspectorMCP creates a new FSInspectorMCP instance.
func NewFSInspectorMCP(provider FileSystemProvider) *FSInspectorMCP {
	return &FSInspectorMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *FSInspectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes contents to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *FSInspectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, path, []byte(content))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *FSInspectorMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	content, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"status": "success", "content": string(content)}, nil
}

func (m *FSInspectorMCP) writeFile(ctx context.Context, path string, content []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, content)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"status": "success"}, nil
}

func (m *FSInspectorMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"status": "success", "entries": entries}, nil
}

// FSProviderFactory creates the appropriate FileSystemProvider based on mode
func FSProviderFactory(isStandalone bool, baseDir string) FileSystemProvider {
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}
