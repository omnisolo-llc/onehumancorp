package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
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
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a query.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"mode":   mode,
			"data":   string(data),
		}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"mode":   mode,
		}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"mode":    mode,
			"entries": entries,
		}, nil
	case "search_files":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		results, err := m.provider.SearchFiles(ctx, claims, query)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"mode":    mode,
			"results": results,
		}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
