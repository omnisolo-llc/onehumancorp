package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system operations.
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
			Description: "Read the contents of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the file to read."}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Write content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the file to write."}, "content": {"type": "string", "description": "Content to write to the file."}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "List the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the directory to list."}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		content, err := m.provider.ReadFile(ctx, path, claims)
		if err != nil {
			return nil, fmt.Errorf("failed to read file: %w", err)
		}

		return map[string]interface{}{
			"status": "success",
			"content": string(content),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		err := m.provider.WriteFile(ctx, path, []byte(contentStr), claims)
		if err != nil {
			return nil, fmt.Errorf("failed to write file: %w", err)
		}

		return map[string]interface{}{
			"status": "success",
			"message": fmt.Sprintf("File %s written successfully.", path),
		}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		entries, err := m.provider.ListDir(ctx, path, claims)
		if err != nil {
			return nil, fmt.Errorf("failed to list directory: %w", err)
		}

		var result []string
		for _, entry := range entries {
			if entry.IsDir() {
				result = append(result, entry.Name() + "/")
			} else {
				result = append(result, entry.Name())
			}
		}

		return map[string]interface{}{
			"status": "success",
			"entries": result,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
