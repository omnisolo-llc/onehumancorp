package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
)

// HybridFSMCP implements the MCP interface for hybrid file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on environment variables.
func NewHybridFSMCP() *HybridFSMCP {
	basePath := os.Getenv("OHC_FS_ROOT")
	if basePath == "" {
		basePath = os.TempDir()
	}

	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	var provider FileSystemProvider
	if isMultiTenant {
		provider = NewCloudFSProvider(basePath)
	} else {
		provider = NewLocalFSProvider(basePath)
	}

	return &HybridFSMCP{provider: provider}
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
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil

	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'pattern' argument")
		}
		matches, err := m.provider.SearchFiles(ctx, path, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"matches": matches}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
