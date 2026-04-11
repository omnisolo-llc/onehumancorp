package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// HybridFSMCP implements the MCP interface for unified file operations.
type HybridFSMCP struct {
	provider mcp.FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on environment.
func NewHybridFSMCP() *HybridFSMCP {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
	fsRoot := os.Getenv("OHC_FS_ROOT")
	if fsRoot == "" {
		fsRoot = os.TempDir()
	}

	var provider mcp.FileSystemProvider
	if isMultiTenant {
		provider = mcp.NewCloudFSProvider(fsRoot)
	} else {
		provider = mcp.NewLocalFSProvider(fsRoot)
	}

	return &HybridFSMCP{
		provider: provider,
	}
}

// NewHybridFSMCPWithProvider creates a new HybridFSMCP with a specific provider (useful for testing).
func NewHybridFSMCPWithProvider(provider mcp.FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available filesystem tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file. Overwrites if it exists.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"content": string(data),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		path := "."
		if p, ok := arguments["path"].(string); ok && p != "" {
			path = p
		}
		entries, err := m.provider.ListDir(ctx, path)
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
			"entries": results,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
