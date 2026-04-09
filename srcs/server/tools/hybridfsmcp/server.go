package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HybridFSMCP implements an MCP server for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance, dynamically choosing the provider based on OHC_STANDALONE.
func NewHybridFSMCP(baseDir string) (*HybridFSMCP, error) {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	var provider FileSystemProvider
	var err error

	if isStandalone {
		provider, err = NewLocalFSProvider(baseDir)
	} else {
		provider, err = NewCloudFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{provider: provider}, nil
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
			Description: "Lists files and directories in a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern in a directory.",
			InputSchema: `{"type": "object", "properties": {"dir": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["dir", "pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

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
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		infos, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var results []map[string]interface{}
		for _, info := range infos {
			results = append(results, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}
		return map[string]interface{}{
			"status":  "success",
			"entries": results,
		}, nil

	case "search_files":
		dir, ok := arguments["dir"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'dir' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		matches, err := m.provider.SearchFiles(ctx, dir, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"matches": matches,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
