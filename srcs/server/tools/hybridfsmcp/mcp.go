package hybridfsmcp

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read a file from the filesystem",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write a file to the filesystem",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List a directory in the filesystem",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"data": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path argument")
		}
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid data argument")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"success": true}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path argument")
		}
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
