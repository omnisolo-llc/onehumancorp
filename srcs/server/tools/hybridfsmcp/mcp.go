package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements an MCP server for hybrid file system operations.
type HybridFSMCP struct {
	provider mcp.FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider mcp.FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file. Data should be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
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
		return m.readFile(ctx, claims, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		data, err := base64.StdEncoding.DecodeString(dataStr)
		if err != nil {
			return nil, fmt.Errorf("invalid base64 data: %w", err)
		}
		return m.writeFile(ctx, claims, path, data)
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, claims, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, claims, path)
	if err != nil {
		return nil, err
	}

	// Check if the provider is cloud by type assertion to see if we need claims
	_, isCloud := m.provider.(*mcp.CloudFSProvider)
	mode := "standalone"
	if isCloud {
		mode = "cloud"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
		"data":   base64.StdEncoding.EncodeToString(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, claims *auth.Claims, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, claims, path, data)
	if err != nil {
		return nil, err
	}

	_, isCloud := m.provider.(*mcp.CloudFSProvider)
	mode := "standalone"
	if isCloud {
		mode = "cloud"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, claims, path)
	if err != nil {
		return nil, err
	}

	_, isCloud := m.provider.(*mcp.CloudFSProvider)
	mode := "standalone"
	if isCloud {
		mode = "cloud"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"entries": entries,
	}, nil
}
