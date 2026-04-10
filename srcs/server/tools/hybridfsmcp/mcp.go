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

// HybridFSMCP implements the MCP interface for the Hybrid File System proxy.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance, instantiating the appropriate provider.
func NewHybridFSMCP(isLocal bool, basePath string) *HybridFSMCP {
	var provider FileSystemProvider
	if isLocal {
		provider = NewLocalFSProvider(basePath)
	} else {
		provider = NewCloudFSProvider(basePath)
	}
	return &HybridFSMCP{provider: provider}
}

// ListTools returns the list of available filesystem tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List files and directories in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name, delegating to the underlying FileSystemProvider.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, errors.New("path argument must be a string")
		}
		return m.provider.ReadFile(ctx, path, claims)

	case "write_file":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, errors.New("path argument must be a string")
		}

		contentIf, ok := arguments["content"]
		if !ok {
			return nil, errors.New("missing content argument")
		}
		content, ok := contentIf.(string)
		if !ok {
			return nil, errors.New("content argument must be a string")
		}
		return m.provider.WriteFile(ctx, path, content, claims)

	case "list_directory":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, errors.New("path argument must be a string")
		}
		return m.provider.ListDir(ctx, path, claims)

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
