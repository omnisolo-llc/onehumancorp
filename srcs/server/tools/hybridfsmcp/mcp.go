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

// HybridFSMCP implements the MCP interface for file system access.
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
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Write data to a file. Overwrites if exists.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "List the files and folders in a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Search for files matching a pattern.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	path, ok := arguments["path"].(string)
	if !ok {
		return nil, errors.New("missing or invalid 'path' argument")
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	switch toolName {
	case "read_file":
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"mode":    mode,
			"content": string(data),
		}, nil

	case "write_file":
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"mode":   mode,
		}, nil

	case "list_directory":
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}

		var results []map[string]interface{}
		for _, e := range entries {
			results = append(results, map[string]interface{}{
				"name":   e.Name,
				"is_dir": e.IsDir,
				"size":   e.Size,
			})
		}
		return map[string]interface{}{
			"status":  "success",
			"mode":    mode,
			"entries": results,
		}, nil

	case "search_files":
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		matches, err := m.provider.SearchFiles(ctx, claims, path, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"mode":    mode,
			"matches": matches,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}