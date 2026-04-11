package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSProviderMCP implements the MCP interface for hybrid file system access.
type HybridFSProviderMCP struct {
	provider FileSystemProvider
}

// NewHybridFSProviderMCP creates a new HybridFSProviderMCP instance based on the environment.
func NewHybridFSProviderMCP(baseDir string) *HybridFSProviderMCP {
	var provider FileSystemProvider
	if os.Getenv("OHC_STANDALONE") == "true" {
		provider = NewLocalFSProvider(baseDir)
	} else {
		provider = NewCloudFSProvider(baseDir)
	}
	return &HybridFSProviderMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSProviderMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the file system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes a file to the file system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory on the file system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSProviderMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	path, ok := arguments["path"].(string)
	if !ok {
		return nil, errors.New("missing or invalid 'path' argument")
	}

	switch toolName {
	case "read_file":
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"data":   string(data),
		}, nil

	case "write_file":
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"entries": entries,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
