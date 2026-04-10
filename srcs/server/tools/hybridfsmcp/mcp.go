package hybridfsmcp

import (
	"context"
	"encoding/base64"
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

// HybridFSMCP implements the MCP interface for unified file system access.
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
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file. Data must be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern within a directory.",
			InputSchema: `{"type": "object", "properties": {"directory": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["directory", "pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	// Require claims in cloud mode
	if !m.provider.IsLocal() {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return nil, errors.New("unauthorized: missing claims in cloud mode")
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
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		data, err := base64.StdEncoding.DecodeString(dataStr)
		if err != nil {
			return nil, fmt.Errorf("failed to decode base64 data: %w", err)
		}
		return m.writeFile(ctx, path, data)
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, path)
	case "search_files":
		directory, ok := arguments["directory"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'directory' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		return m.searchFiles(ctx, directory, pattern)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) getMode() string {
	if m.provider.IsLocal() {
		return "standalone"
	}
	return "cloud"
}

func (m *HybridFSMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"content": base64.StdEncoding.EncodeToString(data), // Return as base64 to handle binary data safely
		"size":    len(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, data)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   m.getMode(),
		"path":   path,
		"size":   len(data),
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

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"results": results,
	}, nil
}

func (m *HybridFSMCP) searchFiles(ctx context.Context, directory string, pattern string) (interface{}, error) {
	files, err := m.provider.SearchFiles(ctx, directory, pattern)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"results": files,
	}, nil
}
