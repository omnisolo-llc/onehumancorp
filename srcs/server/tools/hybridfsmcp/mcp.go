package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HybridFSMCP implements the MCP interface for file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
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
			Description: "Lists files in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
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
		return m.readFile(ctx, claims, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, claims, path, []byte(content))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, claims, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, claims, path)
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

func (m *HybridFSMCP) writeFile(ctx context.Context, claims *auth.Claims, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, claims, path, data)
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

func (m *HybridFSMCP) listDirectory(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, claims, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, e := range entries {
		info, err := e.Info()
		size := int64(0)
		if err == nil {
			size = info.Size()
		}

		results = append(results, map[string]interface{}{
			"name":  e.Name(),
			"is_dir": e.IsDir(),
			"size":  size,
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

// InitProvider initializes the appropriate provider based on the environment.
func InitProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}
