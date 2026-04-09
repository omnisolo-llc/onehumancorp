package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the file system logic.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system.
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
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

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
		content, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil

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
		contentStr, ok := contentIf.(string)
		if !ok {
			return nil, errors.New("content argument must be a string")
		}

		err := m.provider.WriteFile(ctx, claims, path, []byte(contentStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, errors.New("path argument must be a string")
		}

		files, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": files}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
