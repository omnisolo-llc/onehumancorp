package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSMCP implements the MCP server interface for unified file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on the current mode (Standalone vs Cloud).
func NewHybridFSMCP(basePath string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_STANDALONE") == "true" {
		provider, err = NewLocalFSProvider(basePath)
	} else {
		provider, err = NewCloudFSProvider(basePath)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{provider: provider}, nil
}

// ListTools returns the list of available filesystem tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of the given file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes data to the given file, creating it if it doesn't exist.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Returns a list of files and directories within the given path.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Recursively searches for files matching a pattern starting from the given path.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	// In cloud mode, require authentication claims
	if !m.provider.IsLocal() {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return nil, errors.New("unauthorized: missing claims")
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
		return m.writeFile(ctx, path, []byte(dataStr))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, path)
	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		return m.searchFiles(ctx, path, pattern)
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
		"status": "success",
		"mode":   mode,
		"data":   string(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, data)
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
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	files, err := m.provider.ListDir(ctx, path)
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
		"files":  files,
	}, nil
}

func (m *HybridFSMCP) searchFiles(ctx context.Context, path string, pattern string) (interface{}, error) {
	matches, err := m.provider.SearchFiles(ctx, path, pattern)
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
		"matches": matches,
	}, nil
}
