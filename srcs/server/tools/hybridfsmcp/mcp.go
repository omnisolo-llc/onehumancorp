package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for the unified file system tools.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new MCP server.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the given path.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}}}`),
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file at the given path.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"data":{"type":"string"}}}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories under a given path.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}}}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, claims *auth.Claims, name string, args map[string]interface{}) (interface{}, error) {
	switch name {
	case "read_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return string(data), nil
	case "write_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		dataStr, ok := args["data"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'data' argument")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return "success", nil
	case "list_directory":
		path, ok := args["path"].(string)
		if !ok {
			return nil, errors.New("invalid or missing 'path' argument")
		}
		res, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return res, nil
	default:
		return nil, errors.New("tool not found")
	}
}
