package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HybridFSMCP implements the MCP interface for file system access.
type HybridFSMCP struct {
	standalone bool
	local      FileSystemProvider
	cloud      FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(standalone bool, local FileSystemProvider, cloud FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		standalone: standalone,
		local:      local,
		cloud:      cloud,
	}
}

func (m *HybridFSMCP) getProvider() FileSystemProvider {
	if m.standalone {
		return m.local
	}
	return m.cloud
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
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
			Description: "Writes content to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories under a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.standalone {
		return nil, errors.New("unauthorized: missing claims")
	}

	provider := m.getProvider()
	if provider == nil {
		return nil, errors.New("provider not configured")
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, provider, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, provider, path, []byte(content))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, provider, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, provider FileSystemProvider, path string) (interface{}, error) {
	data, err := provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status":  "success",
		"content": string(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, provider FileSystemProvider, path string, data []byte) (interface{}, error) {
	err := provider.WriteFile(ctx, path, data)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, provider FileSystemProvider, path string) (interface{}, error) {
	files, err := provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, f := range files {
		results = append(results, map[string]interface{}{
			"name":   f.Name,
			"is_dir": f.IsDir,
			"size":   f.Size,
		})
	}

	return map[string]interface{}{
		"status": "success",
		"files":  results,
	}, nil
}
