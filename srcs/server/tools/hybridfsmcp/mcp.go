package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP is the MCP server for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of tools provided by the file system MCP.
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
			Description: "Lists files and directories under a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.provider.IsLocal() {
		orgID := auth.OrganizationIDFromContext(ctx)
		if orgID == "" {
			return nil, errors.New("unauthorized: missing organization ID")
		}
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, path, []byte(contentStr))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			path = "." // Default to root
		}
		return m.listDirectory(ctx, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"content": string(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, content []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, content)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
		"path":   path,
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	infos, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, info := range infos {
		results = append(results, map[string]interface{}{
			"name":          info.Name(),
			"size":          info.Size(),
			"is_dir":        info.IsDir(),
			"last_modified": info.ModTime().Format(time.RFC3339),
		})
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"results": results,
	}, nil
}
