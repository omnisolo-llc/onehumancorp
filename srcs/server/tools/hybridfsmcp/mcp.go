package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for FileSystem operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	if provider == nil {
		provider = NewProvider()
	}
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
			Description: "Writes content to a file. Requires 'content' as base64 string.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories at a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	// In Cloud-Native mode, claims are required to identify the tenant.
	// We can let the provider handle the checking of claims based on its implementation.
	// But as an extra safety measure, if it's multi-tenant and there are no claims, reject early.
	if os.Getenv("OHC_MULTITENANT") == "true" && claims == nil {
		return nil, errors.New("unauthorized: missing claims in multi-tenant mode")
	}

	path, ok := arguments["path"].(string)
	if !ok || path == "" {
		return nil, errors.New("missing or invalid 'path' argument")
	}

	switch toolName {
	case "read_file":
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		// Return as base64 string for text/binary safety in JSON
		return map[string]interface{}{
			"status":  "success",
			"content": base64.StdEncoding.EncodeToString(data),
		}, nil

	case "write_file":
		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		data, err := base64.StdEncoding.DecodeString(contentStr)
		if err != nil {
			// fallback to raw string if it's not base64
			data = []byte(contentStr)
		}

		err = m.provider.WriteFile(ctx, claims, path, data)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		entries, err := m.provider.ListDir(ctx, claims, path)
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
