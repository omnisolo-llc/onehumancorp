package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: `{"type": "object", "properties": {"target_path": {"type": "string"}}, "required": ["target_path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: `{"type": "object", "properties": {"target_path": {"type": "string"}, "content": {"type": "string"}}, "required": ["target_path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a directory.",
			InputSchema: `{"type": "object", "properties": {"target_path": {"type": "string"}}, "required": ["target_path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		targetPath, ok := arguments["target_path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'target_path' argument")
		}
		return m.readFile(ctx, targetPath)
	case "write_file":
		targetPath, ok := arguments["target_path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'target_path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, targetPath, []byte(content))
	case "list_directory":
		targetPath, ok := arguments["target_path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'target_path' argument")
		}
		return m.listDirectory(ctx, targetPath)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, targetPath string) (interface{}, error) {
	content, err := m.provider.ReadFile(ctx, targetPath)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"content": string(content),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, targetPath string, content []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, targetPath, content)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status": "success",
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, targetPath string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, targetPath)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"entries": entries,
	}, nil
}
